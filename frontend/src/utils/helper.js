const getPDA = async (program, ido_id, seed) => {
  const idoIdBuff = Buffer.alloc(4);
  idoIdBuff.writeUInt32LE(ido_id, 0);
  const [idoPDAs, bumb] = PublicKey.findProgramAddressSync(
    [
      utils.bytes.utf8.encode(seed),
      provider.wallet.publicKey.toBuffer(),
      idoIdBuff,
    ],
    program.programId
  );
  return idoPDAs;
};

class Helper {
  getPDA(program, ido_id, seed) {
    try {
      const idoIdBuff = Buffer.alloc(4);
      idoIdBuff.writeUInt32LE(ido_id, 0);
      const [idoPDAs, bumb] = PublicKey.findProgramAddressSync(
        [
          utils.bytes.utf8.encode(seed),
          provider.wallet.publicKey.toBuffer(),
          idoIdBuff,
        ],
        program.programId
      );
      return idoPDAs;
    } catch (error) {
      console.log(error);
    }
    return null;
  }
}
