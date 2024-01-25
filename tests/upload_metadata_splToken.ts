import {clusterApiUrl, Connection, Keypair, PublicKey} from "@solana/web3.js";
import {createUmi} from "@metaplex-foundation/umi-bundle-defaults";
import {createMetadataAccountV3} from "@metaplex-foundation/mpl-token-metadata";
import {fromWeb3JsKeypair, fromWeb3JsPublicKey} from "@metaplex-foundation/umi-web3js-adapters";
import {createSignerFromKeypair} from "@metaplex-foundation/umi";
import {base58} from '@metaplex-foundation/umi/serializers';
import * as fs from 'fs';
import util from 'util'
import { web3 } from "@coral-xyz/anchor";
const readFile = util.promisify(fs.readFile);

const uploadMetadataForToken = async (offChainMetadata: any) => {
    const endpoint = clusterApiUrl('devnet');
    let connection = new Connection(endpoint);
    const umi = createUmi(endpoint)

    const web3jsKeyPair = await getWalletFromJson('./id.json')// load your keypair here

    const keypair = fromWeb3JsKeypair(web3jsKeyPair);
    const signer = createSignerFromKeypair(umi, keypair);
    umi.identity = signer;
    umi.payer = signer;

    let CreateMetadataAccountV3Args = {
        //accounts
        mint: fromWeb3JsPublicKey(new PublicKey('your mint here')),
        mintAuthority: signer,
        payer: signer,
        updateAuthority: fromWeb3JsKeypair(web3jsKeyPair).publicKey,
        data: {
            name: offChainMetadata.name,
            symbol: offChainMetadata.symbol,
            uri: offChainMetadata.uri,
            sellerFeeBasisPoints: 0,
            creators: null,
            collection: null,
            uses: null
        },
        isMutable: false,
        collectionDetails: null,
    }

    let instruction = createMetadataAccountV3(
        umi,
        CreateMetadataAccountV3Args
    )

    const transaction = await instruction.buildAndSign(umi);

    const transactionSignature = await umi.rpc.sendTransaction(transaction);
    const signature = base58.deserialize(transactionSignature);
    console.log({signature})
}


(async () => {
    const offChainMetadata = {
        name: "your token name",
        symbol: "⚔️",
        description: "your token description",
        image: "add public URL to image you'd like to use" 
    }
    await uploadMetadataForToken(offChainMetadata);
})()



const getWalletFromJson= async (keyPair: String): Promise<Keypair>=>{
    try {
        //@ts-ignore
        const arr: Uint8Array= Object.values(keyPair._keypair.secretKey)
        const secretKey  = new Uint8Array(arr);
        const wallet = web3.Keypair.fromSecretKey(secretKey);
        console.log(wallet.publicKey.toString());
        return wallet;   
    } catch (error) {
        console.log(error);
        return null;
    }
}