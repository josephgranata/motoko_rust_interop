import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'cyclesBalance' : ActorMethod<[], bigint>,
  'delete' : ActorMethod<[], Principal>,
  'init' : ActorMethod<[], Principal>,
  'installCode' : ActorMethod<[Array<number>, Array<number>], Principal>,
  'storageLoadWasm' : ActorMethod<
    [Array<number>],
    { 'total' : bigint, 'chunks' : bigint },
  >,
  'storateResetWasm' : ActorMethod<[], undefined>,
  'wasmHash' : ActorMethod<[], Array<number>>,
}
