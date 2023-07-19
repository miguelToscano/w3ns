export const idlFactory = ({ IDL }) => {
  const ApiError = IDL.Variant({
    'InterCanisterCallError' : IDL.Text,
    'InsufficientCyclesReceived' : IDL.Text,
    'TopicNotFound' : IDL.Null,
    'ApiKeyNotFound' : IDL.Null,
    'ApiKeyAlreadyExists' : IDL.Null,
    'InvalidApiKey' : IDL.Null,
    'SubscriberNotFound' : IDL.Null,
    'InternalError' : IDL.Null,
    'TopicAlreadyExists' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ApiError });
  const QueuedEmail = IDL.Record({
    'to' : IDL.Text,
    'title' : IDL.Text,
    'api_key' : IDL.Text,
    'body' : IDL.Text,
  });
  const QueuedPush = IDL.Record({
    'title' : IDL.Text,
    'api_key' : IDL.Text,
    'body' : IDL.Text,
    'firebase_token' : IDL.Text,
  });
  const QueuedSms = IDL.Record({
    'to' : IDL.Text,
    'api_key' : IDL.Text,
    'message' : IDL.Text,
  });
  const SendEmailInput = IDL.Record({
    'to' : IDL.Text,
    'title' : IDL.Text,
    'body' : IDL.Text,
  });
  const SendSmsInput = IDL.Record({ 'to' : IDL.Text, 'message' : IDL.Text });
  const SendPushInput = IDL.Record({
    'title' : IDL.Text,
    'body' : IDL.Text,
    'firebase_token' : IDL.Text,
  });
  const ApiKey = IDL.Record({
    'value' : IDL.Text,
    'owner' : IDL.Principal,
    'created_at' : IDL.Nat64,
  });
  const Topic = IDL.Record({
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'created_at' : IDL.Nat64,
    'subscribers' : IDL.Vec(IDL.Text),
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : ApiError });
  const SendPushToTopicInput = IDL.Record({
    'title' : IDL.Text,
    'topic' : IDL.Text,
    'body' : IDL.Text,
  });
  const SubscribeUserToTopicInput = IDL.Record({
    'topic' : IDL.Text,
    'registration_token' : IDL.Text,
  });
  const UnsubscribeUserFromTopic = IDL.Record({
    'topic' : IDL.Text,
    'registration_token' : IDL.Text,
  });
  return IDL.Service({
    'create_topic' : IDL.Func([IDL.Text], [Result], []),
    'cycles' : IDL.Func([], [IDL.Nat64], ['query']),
    'delete_topic' : IDL.Func([IDL.Text], [Result], []),
    'dequeue_email_notifications' : IDL.Func([], [IDL.Vec(QueuedEmail)], []),
    'dequeue_push_notifications' : IDL.Func([], [IDL.Vec(QueuedPush)], []),
    'dequeue_sms_notifications' : IDL.Func([], [IDL.Vec(QueuedSms)], []),
    'enqueue_email_notification' : IDL.Func([SendEmailInput], [Result], []),
    'enqueue_eth_email_notification' : IDL.Func(
        [IDL.Text, SendEmailInput],
        [Result],
        [],
      ),
    'enqueue_eth_sms_notification' : IDL.Func(
        [IDL.Text, SendSmsInput],
        [Result],
        [],
      ),
    'enqueue_push_notification' : IDL.Func([SendPushInput], [Result], []),
    'enqueue_sms_notification' : IDL.Func([SendSmsInput], [Result], []),
    'get_all' : IDL.Func([], [IDL.Vec(ApiKey)], ['query']),
    'get_queued_email_notifications' : IDL.Func(
        [],
        [IDL.Vec(QueuedEmail)],
        ['query'],
      ),
    'get_queued_push_notifications' : IDL.Func(
        [],
        [IDL.Vec(QueuedPush)],
        ['query'],
      ),
    'get_queued_sms_notifications' : IDL.Func(
        [],
        [IDL.Vec(QueuedSms)],
        ['query'],
      ),
    'get_topics' : IDL.Func([], [IDL.Vec(Topic)], ['query']),
    'has_key_registered' : IDL.Func([], [IDL.Bool], ['query']),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'register_eth_key' : IDL.Func([IDL.Text, IDL.Text], [Result], []),
    'register_key' : IDL.Func([IDL.Text], [Result], []),
    'remove_key' : IDL.Func([], [Result], []),
    'send_email' : IDL.Func([SendEmailInput], [Result_1], []),
    'send_push' : IDL.Func([SendPushInput], [Result_1], []),
    'send_push_to_topic' : IDL.Func([SendPushToTopicInput], [Result_1], []),
    'send_sms' : IDL.Func([SendSmsInput], [Result_1], []),
    'subscribe_user_to_topic' : IDL.Func(
        [SubscribeUserToTopicInput],
        [Result],
        [],
      ),
    'unsubscribe_user_from_topic' : IDL.Func(
        [UnsubscribeUserFromTopic],
        [Result],
        [],
      ),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
