import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'create_bucket' : ActorMethod<[Principal], Principal>,
  'greet' : ActorMethod<[string], string>,
}
