import { BN } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

export interface IdoAccount{
    idoId: number,
    rate: number,
    openTimestamp: number,
    cap: BN,
    participated: BN,
    participatedCount: number,
    closed: boolean,
    releaseToken: PublicKey,
    releaseTokenPair?: PublicKey,
    raiseToken: PublicKey,
    releaseTokenDecimals: number,
    raiseTokenDecimals: number,
    authority: PublicKey,
    tiers: Array<TierItem>,
    rounds: Array<RoundItem>,
    releases: Array<ReleaseItem>,
}
export interface TierItem {
    name: String,
    allocationsCount: number
}
export interface ReleaseItem {
    fromTimestamp: number,
    toTimestamp: number,
    percent: number,
} 
export interface RoundItem{
     name: String,
     durationSeconds: number,
     class: RoundClass,
     tierAllocations: Array<BN>,
}
export enum RoundClass {
    Allocation,
    FcfsPrepare,
    Fcfs,
}

export interface UserStraitPda {
     address: PublicKey, //16
     tierIndex: number, //1
     allocated: boolean, //1
     participateAmount: BN, //16
     claimAmount: BN, //16
     owner: PublicKey,//32
}