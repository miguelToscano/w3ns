import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type ApiError = { 'InterCanisterCallError' : string } |
  { 'InsufficientCyclesReceived' : string } |
  { 'TopicNotFound' : null } |
  { 'ApiKeyNotFound' : null } |
  { 'ApiKeyAlreadyExists' : null } |
  { 'InvalidApiKey' : null } |
  { 'SubscriberNotFound' : null } |
  { 'InternalError' : null } |
  { 'TopicAlreadyExists' : null };
export interface ApiKey {
  'value' : string,
  'owner' : Principal,
  'created_at' : bigint,
}
export interface QueuedEmail {
  'to' : string,
  'title' : string,
  'api_key' : string,
  'body' : string,
}
export interface QueuedPush {
  'title' : string,
  'api_key' : string,
  'body' : string,
  'firebase_token' : string,
}
export interface QueuedSms {
  'to' : string,
  'api_key' : string,
  'message' : string,
}
export type Result = { 'Ok' : null } |
  { 'Err' : ApiError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : ApiError };
export interface SendEmailInput {
  'to' : string,
  'title' : string,
  'body' : string,
}
export interface SendPushInput {
  'title' : string,
  'body' : string,
  'firebase_token' : string,
}
export interface SendPushToTopicInput {
  'title' : string,
  'topic' : string,
  'body' : string,
}
export interface SendSmsInput { 'to' : string, 'message' : string }
export interface SubscribeUserToTopicInput {
  'topic' : string,
  'registration_token' : string,
}
export interface Topic {
  'owner' : Principal,
  'name' : string,
  'created_at' : bigint,
  'subscribers' : Array<string>,
}
export interface UnsubscribeUserFromTopic {
  'topic' : string,
  'registration_token' : string,
}
export interface _SERVICE {
  'create_topic' : ActorMethod<[string], Result>,
  'cycles' : ActorMethod<[], bigint>,
  'delete_topic' : ActorMethod<[string], Result>,
  'dequeue_email_notifications' : ActorMethod<[], Array<QueuedEmail>>,
  'dequeue_push_notifications' : ActorMethod<[], Array<QueuedPush>>,
  'dequeue_sms_notifications' : ActorMethod<[], Array<QueuedSms>>,
  'enqueue_email_notification' : ActorMethod<[SendEmailInput], Result>,
  'enqueue_eth_email_notification' : ActorMethod<
    [string, SendEmailInput],
    Result
  >,
  'enqueue_eth_sms_notification' : ActorMethod<[string, SendSmsInput], Result>,
  'enqueue_push_notification' : ActorMethod<[SendPushInput], Result>,
  'enqueue_sms_notification' : ActorMethod<[SendSmsInput], Result>,
  'get_all' : ActorMethod<[], Array<ApiKey>>,
  'get_queued_email_notifications' : ActorMethod<[], Array<QueuedEmail>>,
  'get_queued_push_notifications' : ActorMethod<[], Array<QueuedPush>>,
  'get_queued_sms_notifications' : ActorMethod<[], Array<QueuedSms>>,
  'get_topics' : ActorMethod<[], Array<Topic>>,
  'has_key_registered' : ActorMethod<[], boolean>,
  'name' : ActorMethod<[], string>,
  'register_eth_key' : ActorMethod<[string, string], Result>,
  'register_key' : ActorMethod<[string], Result>,
  'remove_key' : ActorMethod<[], Result>,
  'send_email' : ActorMethod<[SendEmailInput], Result_1>,
  'send_push' : ActorMethod<[SendPushInput], Result_1>,
  'send_push_to_topic' : ActorMethod<[SendPushToTopicInput], Result_1>,
  'send_sms' : ActorMethod<[SendSmsInput], Result_1>,
  'subscribe_user_to_topic' : ActorMethod<[SubscribeUserToTopicInput], Result>,
  'unsubscribe_user_from_topic' : ActorMethod<
    [UnsubscribeUserFromTopic],
    Result
  >,
  'whoami' : ActorMethod<[], Principal>,
}
