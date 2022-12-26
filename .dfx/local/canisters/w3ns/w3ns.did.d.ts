import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ApiKey {
  'value' : string,
  'owner' : Principal,
  'created_at' : bigint,
}
export type Result = { 'Ok' : null } |
  { 'Err' : null };
export interface _SERVICE {
  'get_all' : ActorMethod<[], Array<ApiKey>>,
  'has_key_registered' : ActorMethod<[], boolean>,
  'name' : ActorMethod<[], string>,
  'register_key' : ActorMethod<[string], Result>,
  'remove_key' : ActorMethod<[], Result>,
  'whoami' : ActorMethod<[], Principal>,
}
