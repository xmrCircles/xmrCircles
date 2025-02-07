# Monero Circles
Multisig Messaging Revamped

## The Problem:
Multisig is hard to use, the MMS is bad. There is no GUI to use multisig and I can't easily exchange keys with large groups of people.

Monero Circles is a new implementation with a GUI that does messaging over Tor to exchange multisig keys, it's not a main wallet, it's a multisig key manager and social application.

Tech Stack: Rust, Arti (Tor), PGP, Tauri, HTML, CSS, Monero (Either a Rust dependency or monero-ts, depending which one has the best Multisig Support), 
It's a Cross platform Desktop Application .


## How it works?

Circles are groups of Nodes connecting to each other. 
Nodes are individual instances of the Application.
Each node has PGP keys to identify itself, a dedicated onion address and XMR Wallet for Multisig.
Nodes join in a circle to exchange Multisig data with each other.

A Circle can be created to have a leader or they are leaderless.

The differences are in message routing and node connection.

A circle leader node will route all messages to it's peers directly, more similar to a server-client relationship. A circle with a leader has a fixed size and can only grow if the leader allows it.

A leaderless circle employs a gossip protocol to pass messages with the circle members. The circle's size can grow as each member can invite new Nodes to join.

Node discovery on the Tor network is either `open` or `restricted` , nodes can control who can connect to them directly and be undetectable completely. Nodes with restricted access must manually add nodes they can connect to.
Node connection can also be restricted via PGP Pubkey whitelists.

Example of a Circle with a Leader, All Peer Nodes connect to the Leader.

```
        Peer
      /
Leader -- Peer
      \     
        Peer

```

Example leaderless Circle, All the nodes are connected and they gossip.

```
      Peer
    /     \
Peer - | - Peer
    \      /
      Peer 
```


Peer communicate using PGP signed messages (JSON). They can join a circle using an invite, which is a signed message either created by a leader, or peers.

In a leaderless circle, peers have consensus using Voting, Peers need to agree to sign invites, they can vote on creating multisig accounts and vote to approve spending transactions. Votes are proposed by temporary leaders.
Voting can be manual or automated and it should be configurable.

There can be multiple Multisig accounts inside a single circle, depending on it's purpose.

The messaging process is the following:

Circle with leader:

1. Alice PGP signs an invite and shares it on another channel. the invite contains an onion address and an identifier for the circle.
2. Bob connects to the onion address and requests to join the circle.
3. Alice and Bob Exchange PGP Public Keys and from now on communicates with encrypted and signed messages only.
4. Charlie joins the same way as Bob. Charlie Exchanges Public Keys with Bob after Alice tells him Bob is in the Circle.
5. Alice prompts Bob and Charlie to create a 2-3 Multisig wallet.
6. Bob and Charlie gossip with each other with signed messages to confirm the data Alice is sending them as they exchange multisig and verify they possess the same public address and private view key.
7. When creating a spending transaction, Alice proposes a Poll to Vote on, which must be signed by Bob and Charlie to come to consensus about the decision.
8. Alice will create the transaction and after the required parties sign it, submits it to the network.


Circle without a leader:
A leaderless circle can grow arbitrarily large. Participants have to chose manually who they want to interact with inside the circle. Inside the leaderless circle, every member buffers and gossips all messages in-case some of the members are down.

1. Circle members agree to create an invite and multiple peers sign it.
2. Then the invite can be used by Bob to establish connection with the signers of the invite.
3. The circle members gossip to Bob to notify him about other Circle members and Bob can request to connect to them by forwarding signed messages. Nodes introduce new nodes to their existing peers.
4. When a multisg wallet is created, a temporary leader is formed who coordinates it, but all the messages (encrypted) are propagated to all the peers who can forward it. 
5. A peer can propose a spend by creating a Poll and then become a temporary leader to organize the transaction signing and dispatch the transaction.

Moderating a circle:
A circle with a leader can chose to kick a peer out, by simply stopping all communication with them and putting them in a blacklist.
For a circle without the leader, blacklists can be used and a poll to come to consensus about who to block.


## Use cases

A Circle is a group of people who can use it to pass multisig messages.

They can use it to create a shared account between 2 people, manage consensus accounts for a small group of people, threshold accounts for an escrow or create any arbitrary multisig account.
It can be used to create a DAO where a group manages spending from a "pool" or it could provide Multisig for p2p trading.

The application also has a UI for PGP, to sign, encrypt messages which can be shared on other platforms, to make sure the person you are chatting with and the circle member is the same.
And it has a key manager to manage lists of PGP pubkeys, manage the circle etc...

It implements a monero wallet only for creating multisig, not for the purpose of a primary personal wallet, for that there are lot of other solutions already.