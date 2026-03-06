// ══════════════════════════════════════════
// TALLER MECÁNICO — Cliente completo
// ══════════════════════════════════════════
const owner = pg.wallet.keypair;

console.log("🚗 Iniciando simulación del Taller Mecánico...");
console.log("Wallet Dueño:", owner.publicKey.toString());

// PDA del taller
const [tallerPda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("taller"), owner.publicKey.toBuffer()],
  pg.program.programId
);
console.log("📍 PDA del Taller:", tallerPda.toString());

async function main() {
  try {

    // ══════════════════════════════════════
    // 1️⃣ CREAR EL TALLER
    // ══════════════════════════════════════
    console.log("\n1️⃣ Creando taller en la blockchain...");
    const tx1 = await pg.program.methods
      .crearTallerMecanico("Taller El Turbo")
      .accounts({
        tallerMecanico: tallerPda,
        owner: owner.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();
    console.log("✅ Taller creado! Hash:", tx1);

    // ══════════════════════════════════════
    // 2️⃣ AGREGAR CARRO
    // ══════════════════════════════════════
    console.log("\n2️⃣ Registrando carro...");
    const tx2 = await pg.program.methods
      .agregarCarro(
        "Juan Perez",
        "ABC-123",
        "Toyota",
        "Corolla",
        2021
      )
      .accounts({
        tallerMecanico: tallerPda,
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();
    console.log("✅ Carro registrado! Hash:", tx2);

    // ══════════════════════════════════════
    // 3️⃣ AGREGAR MECÁNICO
    // ══════════════════════════════════════
    console.log("\n3️⃣ Registrando mecanico...");
    const tx3 = await pg.program.methods
      .agregarMecanico("Carlos Lopez", "Motor y frenos")
      .accounts({
        tallerMecanico: tallerPda,
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();
    console.log("✅ Mecanico registrado! Hash:", tx3);

    // ══════════════════════════════════════
    // 4️⃣ CREAR ORDEN DE SERVICIO
    // ══════════════════════════════════════
    console.log("\n4️⃣ Creando orden de servicio...");
    const tx4 = await pg.program.methods
      .crearOrden(
        "ABC-123",
        "Cambio de aceite",
        new BN(500000000)
      )
      .accounts({
        tallerMecanico: tallerPda,
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();
    console.log("✅ Orden creada! Hash:", tx4);

    // ══════════════════════════════════════
    // 5️⃣ ASIGNAR MECÁNICO A LA ORDEN
    // ══════════════════════════════════════
    console.log("\n5️⃣ Asignando mecanico a la orden...");
    const tx5 = await pg.program.methods
      .asignarMecanico("ABC-123", "Carlos Lopez")
      .accounts({
        tallerMecanico: tallerPda,
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();
    console.log("✅ Mecanico asignado! Hash:", tx5);

    // ══════════════════════════════════════
    // 6️⃣ CAMBIAR ESTADO → EN PROCESO
    // ══════════════════════════════════════
    console.log("\n6️⃣ Cambiando estado a En Proceso...");
    const tx6 = await pg.program.methods
      .cambiarEstadoOrden("ABC-123", { enProceso: {} })
      .accounts({
        tallerMecanico: tallerPda,
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();
    console.log("✅ Estado: En Proceso! Hash:", tx6);

    // ══════════════════════════════════════
    // 7️⃣ CAMBIAR ESTADO → TERMINADO
    // ══════════════════════════════════════
    console.log("\n7️⃣ Cambiando estado a Terminado...");
    const tx7 = await pg.program.methods
      .cambiarEstadoOrden("ABC-123", { terminado: {} })
      .accounts({
        tallerMecanico: tallerPda,
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();
    console.log("✅ Estado: Terminado! Hash:", tx7);

    // ══════════════════════════════════════
    // 8️⃣ GUARDAR EN HISTORIAL
    // ══════════════════════════════════════
    console.log("\n8️⃣ Guardando en historial de reparaciones...");
    const tx8 = await pg.program.methods
      .agregarReparacion(
        "ABC-123",
        "Cambio de aceite",
        new BN(500000000),
        "Carlos Lopez"
      )
      .accounts({
        tallerMecanico: tallerPda,
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();
    console.log("✅ Reparacion guardada! Hash:", tx8);

    // ══════════════════════════════════════
    // 9️⃣ AUDITORÍA — LEER LA BLOCKCHAIN
    // ══════════════════════════════════════
    console.log("\n🔍 Leyendo datos de la blockchain...");
    const taller = await pg.program.account.tallerMecanico.fetch(tallerPda);

    console.log("\n══════════════════════════════════════");
    console.log("   DATOS INMUTABLES EN LA BLOCKCHAIN");
    console.log("══════════════════════════════════════");
    console.log("🏪 Taller    :", taller.nombre);
    console.log("👤 Owner     :", taller.owner.toString());
    console.log("🚗 Carros    :", taller.carros.length);
    console.log("📄 Ordenes   :", taller.ordenes.length);
    console.log("🔧 Mecanicos :", taller.mecanicos.length);

    console.log("\n🚗 Carro registrado:");
    const carro = taller.carros[0];
    console.log("  Propietario  :", carro.propietario);
    console.log("  Placa        :", carro.placa);
    console.log("  Marca/Modelo :", carro.marca, carro.modelo);
    console.log("  Año          :", carro.anio.toString());
    console.log("  Reparaciones :", carro.reparaciones.length);

    console.log("\n📄 Orden de servicio:");
    const orden = taller.ordenes[0];
    console.log("  Placa        :", orden.placa);
    console.log("  Descripcion  :", orden.descripcion);
    console.log("  Costo        :", orden.costo.toString(), "lamports");
    console.log("  Mecanico     :", orden.mecanico);
    console.log("  Pagado       :", orden.pagado);

    console.log("\n🔧 Mecanico:");
    const mecanico = taller.mecanicos[0];
    console.log("  Nombre       :", mecanico.nombre);
    console.log("  Especialidad :", mecanico.especialidad);
    console.log("  Activo       :", mecanico.activo);
    console.log("══════════════════════════════════════");

  } catch (error) {
    console.error("❌ Error en la simulacion:", error);
  }
}

main();
