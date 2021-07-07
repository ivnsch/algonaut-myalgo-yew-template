# Algonaut + My Algo + Yew template

Template to sign [Algonaut](https://github.com/manuelmauro/algonaut) transactions with [My Algo wallet](https://github.com/randlabs/myalgo-connect) in a [Yew](https://github.com/yewstack/yew) application.

![ScreenShot](screen/screenr.gif)

## Pre-requisites

Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Cargo `wasm-pack`

```
cargo install wasm-pack
```

[Node.js](https://nodejs.org/en/)

## Initialize

Fetch dependencies:

    npm install

## Run local developer setup

Start a local development server, which re-builds every time you make changes to the code:

    npm run start:dev

Direct your web browser to: http://localhost:8000

## Perform a release build

To build the Rust components and package up the NPM dependencies, run:

    npm run build

The release is in the `dist/` folder.

## Algorand network setup

### Update endpoint

Update [dependencies.rs](https://github.com/i-schuetz/algonaut-myalgo-yew-template/blob/master/src/dependencies.rs) with your node's URL.

### Testing with sandbox or private network

To sign transactions for a private network or sandbox using My Algo, you just have to share an account between them. The network selected on My Algo doesn't matter (you probably want to use TestNet, for general bookkeeping).

To do this, you can either export the mnemonic from your node:

```
goal account export -a <ADDRESS> -d <node directory>
```

and import it in My Algo, or the other way, import the mnemonic from My Algo:

```
goal account import -m "<MNEMONIC>" -d <node directory>
```

Also, ensure that the account on the sandbox / private network is sufficiently funded:

```
goal clerk send -a <AMOUNT> -f <SENDER_ADDRESS> -t <RECEIVER_ADDRESS> -d <node directory>
```

You can alternatively perform these steps with the SDK.

## Acknowledgments

Project configuration and instructions based on [patternfly-yew-quickstart](https://github.com/ctron/patternfly-yew-quickstart)

## Related

[Minimal Algonaut and Yew integration](https://github.com/i-schuetz/algorand-yew-example) (no wallet)
