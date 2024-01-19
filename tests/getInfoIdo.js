const { PublicKey } = require("@solana/web3.js");



const getInfoIdoAccount = async (program, idoAccountAddress)=>{
    const idoAccountPub  = new PublicKey(idoAccountAddress)
    let ido_info = await program.account.idoAccountInfo.fetch(idoAccountPub);
    let xx = ido_info.toString();
    console.log(xx);
    return ido_info
}

module.exports = {
    getInfoIdoAccount 
}