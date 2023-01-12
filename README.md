# W3NS
An Internet Computer service for notifications

# Getting an API key
We use Courier notifications API in order to integrate with different providers.

- Create an account in https://www.courier.com/
- Sign in
- Go to Settings > API Keys
- Generate New API key

# Register your API Key
Now you need to register your API key in order to let W3NS send notifications on your behalf

`dfx canister --wallet=<YOUR_WALLET_PRINCIPAL_ID> call w3ns register_key "(\"<YOUR_COURIER_API_KEY>\")"`

# Courier Channels setup
W3NS currently supports 3 notifications channels:

- `Gmail` for emails 
- `Twilio` for SMS
- `Firebase FCM` for push

To set up a Courier Channel:

- Sign in to https://www.courier.com/
- Go to Channels > Add Channel
- Follow the steps for each of the providers that W3NS supports (Gmail, Twilio, Firebase FCM)

# Sending notifications
For the following steps to work you should already have you Courier's API key registered as well as the Courier's providers configured.
Each notification sending method requires 4000000000 cycles payment in order to cover for cycles consumed.
## Send email
You can send emails via W3NS by running:

`dfx canister --wallet=<YOUR_WALLET_PRINCIPAL_ID> call --with-cycles=4000000000 w3ns send_email "(record { to=\"<EMAIL>\"; subject=\"<SUBJECT>\"; body=\"<BODY>\" })"`

## Send SMS
You can send SMS via W3NS by running

`dfx canister --wallet=<YOUR_WALLET_PRINCIPAL_ID> call --with-cycles=4000000000 w3ns send_sms "(record { to=\"<PHONE_NUMBER>\";     message=\"<MESSAGE>\" })"`

## Send Push
You can send Push notifications via W3NS by running

`dfx canister --wallet=<YOUR_WALLET_PRINCIPAL_ID> call --with-cycles=4000000000 w3ns send_push "(record { to=\"<FIREBASE_TOKEN>\"; title=\"<SUBJECT>\"; body=\"<BODY>\" })"`

## Send push to topic
To send a push notifcations to users subscribed to a certain topic first you have to create it and subscribe users to it

### Create a topic
`dfx canister --wallet=<YOUR_WALLET_PRINCIPAL_ID> call w3ns create_topic "(\"<YOUR_TOPIC_NAME>\")"`

### Subscribe a user to a topic
`dfx canister --wallet=<YOUR_WALLET_PRINCIPAL_ID> call w3ns subscribe_user_to_topic "(record { topic=\"<YOUR_TOPIC_NAME>\"; registration_token=\"<USERS_FIREBASE_TOKEN>\" })"`

### Send push notification to users subscribed to a topic
`dfx canister --wallet=<YOUR_WALLET_PRINCIPAL_ID> call --with-cycles=4000000000 w3ns send_push_to_topic "(record { title=\"<TITLE>\"; body=\"<BODY>\"; topic=\"<TOPIC_NAME>\" })"`

### Unsubscribe user from a topic
`dfx canister --wallet=<YOUR_WALLET_PRINCIPAL_ID> call w3ns unsubscribe_user_from_topic "(record { topic=\"<YOUR_TOPIC_NAME>\"; registration_token=\"<USERS_FIREBASE_TOKEN>\" })"`
