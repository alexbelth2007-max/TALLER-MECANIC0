use anchor_lang::prelude::*;

declare_id!("4inU6VykqQTu5scvjwkn1kvMMEXVN6qWKzyQUnzK5r45");

#[program]
pub mod taller_mecanico_db {
    use super::*;

    pub fn crear_taller_mecanico(
        ctx: Context<CrearTallerMecanico>,
        nombre: String,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;
        taller.nombre    = nombre;
        taller.owner     = *ctx.accounts.owner.key;
        taller.carros    = Vec::new();
        taller.ordenes   = Vec::new();
        taller.mecanicos = Vec::new();
        Ok(())
    }

    pub fn agregar_carro(
        ctx:         Context<ModificarTaller>,
        propietario: String,
        placa:       String,
        marca:       String,
        modelo:      String,
        anio:        u16,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        let nuevo_carro = Carro {
            propietario,
            placa,
            marca,
            modelo,
            anio,
            reparaciones: Vec::new(),
        };

        taller.carros.push(nuevo_carro);
        Ok(())
    }

    pub fn eliminar_carro(
        ctx:   Context<ModificarTaller>,
        placa: String,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        if let Some(pos) = taller.carros.iter().position(|c| c.placa == placa) {
            taller.carros.swap_remove(pos);
            Ok(())
        } else {
            Err(ErrorCode::CarroNoEncontrado.into())
        }
    }

    pub fn ver_carros(ctx: Context<VerTaller>) -> Result<()> {
        let taller = &ctx.accounts.taller_mecanico;

        if taller.carros.is_empty() {
            msg!("No hay carros registrados en el taller.");
            return Ok(());
        }

        msg!("Listado de carros en el taller '{}':", taller.nombre);

        for (i, carro) in taller.carros.iter().enumerate() {
            msg!(
                "Carro #{}: Placa: {}, Marca: {}, Modelo: {}, Año: {}, Propietario: {}",
                i + 1,
                carro.placa,
                carro.marca,
                carro.modelo,
                carro.anio,
                carro.propietario,
            );
        }
        Ok(())
    }

    pub fn crear_orden(
        ctx:         Context<ModificarTaller>,
        placa:       String,
        descripcion: String,
        costo:       u64,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        let carro_existe = taller.carros.iter().any(|c| c.placa == placa);
        if !carro_existe {
            return Err(ErrorCode::CarroNoEncontrado.into());
        }

        let nueva_orden = OrdenServicio {
            placa,
            descripcion,
            costo,
            estado:   EstadoOrden::Pendiente,
            mecanico: String::new(),
            pagado:   false,
        };

        taller.ordenes.push(nueva_orden);
        Ok(())
    }

    pub fn ver_ordenes(ctx: Context<VerTaller>) -> Result<()> {
        let taller = &ctx.accounts.taller_mecanico;

        if taller.ordenes.is_empty() {
            msg!("No hay ordenes de servicio en el taller.");
            return Ok(());
        }

        msg!("Ordenes de servicio en el taller '{}':", taller.nombre);

        for (i, orden) in taller.ordenes.iter().enumerate() {
            let estado_str = match orden.estado {
                EstadoOrden::Pendiente => "Pendiente",
                EstadoOrden::EnProceso => "En Proceso",
                EstadoOrden::Terminado => "Terminado",
            };
            msg!(
                "Orden #{}: Placa: {}, Desc: {}, Costo: {}, Estado: {}, Mecanico: {}, Pagado: {}",
                i + 1,
                orden.placa,
                orden.descripcion,
                orden.costo,
                estado_str,
                if orden.mecanico.is_empty() { "Sin asignar" } else { &orden.mecanico },
                orden.pagado,
            );
        }
        Ok(())
    }

    pub fn cambiar_estado_orden(
        ctx:          Context<ModificarTaller>,
        placa:        String,
        nuevo_estado: EstadoOrden,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        if let Some(orden) = taller.ordenes.iter_mut().find(|o| o.placa == placa) {
            orden.estado = nuevo_estado;
            msg!("Estado de la orden para '{}' actualizado.", placa);
            Ok(())
        } else {
            Err(ErrorCode::OrdenNoEncontrada.into())
        }
    }

    pub fn eliminar_orden(
        ctx:   Context<ModificarTaller>,
        placa: String,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        if let Some(pos) = taller.ordenes.iter().position(|o| o.placa == placa) {
            taller.ordenes.swap_remove(pos);
            msg!("Orden para el carro '{}' eliminada.", placa);
            Ok(())
        } else {
            Err(ErrorCode::OrdenNoEncontrada.into())
        }
    }

    pub fn agregar_mecanico(
        ctx:          Context<ModificarTaller>,
        nombre:       String,
        especialidad: String,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        let nuevo_mecanico = Mecanico {
            nombre,
            especialidad,
            activo: true,
        };

        taller.mecanicos.push(nuevo_mecanico);
        Ok(())
    }

    pub fn ver_mecanicos(ctx: Context<VerTaller>) -> Result<()> {
        let taller = &ctx.accounts.taller_mecanico;

        if taller.mecanicos.is_empty() {
            msg!("No hay mecanicos registrados en el taller.");
            return Ok(());
        }

        msg!("Mecanicos en el taller '{}':", taller.nombre);

        for (i, mecanico) in taller.mecanicos.iter().enumerate() {
            msg!(
                "Mecanico #{}: Nombre: {}, Especialidad: {}, Activo: {}",
                i + 1,
                mecanico.nombre,
                mecanico.especialidad,
                mecanico.activo,
            );
        }
        Ok(())
    }

    pub fn asignar_mecanico(
        ctx:             Context<ModificarTaller>,
        placa:           String,
        nombre_mecanico: String,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        let mecanico_activo = taller
            .mecanicos
            .iter()
            .any(|m| m.nombre == nombre_mecanico && m.activo);

        if !mecanico_activo {
            return Err(ErrorCode::MecanicoNoEncontrado.into());
        }

        if let Some(orden) = taller.ordenes.iter_mut().find(|o| o.placa == placa) {
            orden.mecanico = nombre_mecanico.clone();
            msg!("Mecanico '{}' asignado al carro '{}'.", nombre_mecanico, placa);
            Ok(())
        } else {
            Err(ErrorCode::OrdenNoEncontrada.into())
        }
    }

    pub fn cambiar_estado_mecanico(
        ctx:             Context<ModificarTaller>,
        nombre_mecanico: String,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        if let Some(mecanico) = taller.mecanicos.iter_mut().find(|m| m.nombre == nombre_mecanico) {
            mecanico.activo = !mecanico.activo;
            msg!(
                "Mecanico '{}' ahora esta: {}",
                nombre_mecanico,
                if mecanico.activo { "Activo" } else { "Inactivo" }
            );
            Ok(())
        } else {
            Err(ErrorCode::MecanicoNoEncontrado.into())
        }
    }

    pub fn agregar_reparacion(
        ctx:         Context<ModificarTaller>,
        placa:       String,
        descripcion: String,
        costo:       u64,
        mecanico:    String,
    ) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        if let Some(carro) = taller.carros.iter_mut().find(|c| c.placa == placa) {
            let nueva_reparacion = Reparacion {
                descripcion,
                costo,
                mecanico,
            };
            carro.reparaciones.push(nueva_reparacion);
            msg!("Reparacion agregada al historial del carro '{}'.", placa);
            Ok(())
        } else {
            Err(ErrorCode::CarroNoEncontrado.into())
        }
    }

    pub fn ver_historial(ctx: Context<VerTaller>, placa: String) -> Result<()> {
        let taller = &ctx.accounts.taller_mecanico;

        if let Some(carro) = taller.carros.iter().find(|c| c.placa == placa) {
            if carro.reparaciones.is_empty() {
                msg!("El carro '{}' no tiene reparaciones registradas.", placa);
                return Ok(());
            }
            msg!("Historial del carro '{}':", placa);
            for (i, rep) in carro.reparaciones.iter().enumerate() {
                msg!(
                    "Reparacion #{}: {}, Costo: {}, Mecanico: {}",
                    i + 1,
                    rep.descripcion,
                    rep.costo,
                    rep.mecanico,
                );
            }
            Ok(())
        } else {
            Err(ErrorCode::CarroNoEncontrado.into())
        }
    }

    pub fn pagar_orden(ctx: Context<PagarOrden>, placa: String) -> Result<()> {
        let taller = &mut ctx.accounts.taller_mecanico;

        let orden = taller
            .ordenes
            .iter_mut()
            .find(|o| o.placa == placa)
            .ok_or(ErrorCode::OrdenNoEncontrada)?;

        if orden.pagado {
            return Err(ErrorCode::OrdenYaPagada.into());
        }
        if orden.estado != EstadoOrden::Terminado {
            return Err(ErrorCode::OrdenNoTerminada.into());
        }

        let costo = orden.costo;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.cliente.to_account_info(),
                to:   ctx.accounts.owner.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_ctx, costo)?;

        orden.pagado = true;
        msg!("Pago de {} lamports por el carro '{}'.", costo, placa);
        Ok(())
    }
}


#[error_code]
pub enum ErrorCode {
    #[msg("No se encontro ningun carro con esa placa.")]
    CarroNoEncontrado,
    #[msg("No se encontro ninguna orden para esa placa.")]
    OrdenNoEncontrada,
    #[msg("No se encontro ningun mecanico con ese nombre.")]
    MecanicoNoEncontrado,
    #[msg("Esta orden ya fue pagada.")]
    OrdenYaPagada,
    #[msg("Solo se pueden pagar ordenes con estado Terminado.")]
    OrdenNoTerminada,
}


#[derive(Accounts)]
pub struct CrearTallerMecanico<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + TallerMecanico::INIT_SPACE,
        seeds = [b"taller", owner.key().as_ref()],
        bump
    )]
    pub taller_mecanico: Account<'info, TallerMecanico>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarTaller<'info> {
    #[account(mut)]
    pub taller_mecanico: Account<'info, TallerMecanico>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerTaller<'info> {
    pub taller_mecanico: Account<'info, TallerMecanico>,
}

#[derive(Accounts)]
pub struct PagarOrden<'info> {
    #[account(mut)]
    pub taller_mecanico: Account<'info, TallerMecanico>,
    #[account(mut)]
    pub cliente: Signer<'info>,
    /// CHECK: Owner del taller, destino del pago
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct TallerMecanico {
    #[max_len(50)]
    pub nombre: String,          // 54 bytes

    pub owner: Pubkey,           // 32 bytes

    #[max_len(5)]
    pub carros: Vec<Carro>,      // 5 carros máx

    #[max_len(5)]
    pub ordenes: Vec<OrdenServicio>, // 5 órdenes máx

    #[max_len(5)]
    pub mecanicos: Vec<Mecanico>,    // 5 mecánicos máx
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Carro {
    #[max_len(30)]
    pub propietario: String,
    #[max_len(10)]
    pub placa: String,
    #[max_len(20)]
    pub marca: String,
    #[max_len(20)]
    pub modelo: String,
    pub anio: u16,
    #[max_len(3)]
    pub reparaciones: Vec<Reparacion>, // 3 reparaciones máx
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct OrdenServicio {
    #[max_len(10)]
    pub placa: String,
    #[max_len(100)]
    pub descripcion: String,
    pub costo: u64,
    pub estado: EstadoOrden,
    #[max_len(30)]
    pub mecanico: String,
    pub pagado: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum EstadoOrden {
    Pendiente,
    EnProceso,
    Terminado,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Mecanico {
    #[max_len(30)]
    pub nombre: String,
    #[max_len(30)]
    pub especialidad: String,
    pub activo: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Reparacion {
    #[max_len(100)]
    pub descripcion: String,
    pub costo: u64,
    #[max_len(30)]
    pub mecanico: String,
}
