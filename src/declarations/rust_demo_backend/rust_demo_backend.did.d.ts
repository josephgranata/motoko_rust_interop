import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AssetKey {
  'token' : [] | [string],
  'sha256' : [] | [Array<number>],
  'name' : string,
  'fullPath' : string,
  'folder' : string,
}
export interface Chunk { 'content' : Array<number>, 'batchId' : bigint }
export interface CommitBatch {
  'headers' : Array<[string, string]>,
  'chunkIds' : Array<bigint>,
  'batchId' : bigint,
}
export interface InitUpload { 'batchId' : bigint }
export interface UploadChunk { 'chunkId' : bigint }
export interface _SERVICE {
  'commitUpload' : ActorMethod<[CommitBatch], undefined>,
  'initUpload' : ActorMethod<[AssetKey], InitUpload>,
  'transferFreezingThresholdCycles' : ActorMethod<[], undefined>,
  'uploadChunk' : ActorMethod<[Chunk], UploadChunk>,
}
