import TCP from "libp2p-tcp";
import Mplex from "libp2p-mplex";
import { NOISE } from "libp2p-noise";
import Libp2p from "libp2p";
import KadDHT from "libp2p-kad-dht";
import delay from "delay"

await main();

async function main() {
    const node = await createNode();

    await delay(1000);
    
    const [/**localhost*/, /**routerhost*/, publichost] = node.multiaddrs;
    if (!publichost) {
        console.error("TCP publichost not found: node.multiaddrs:", node.multiaddrs);
        return
    }

    console.log(`${publichost.toString()}/p2p/${node.peerId.toB58String()}`);

    await node.stop();
}

/**
 * function createNode()
 */
async function createNode() {
    const node = await Libp2p.create({
        addresses: {
            listen: ['/ip4/0.0.0.0/tcp/0']
        },
        modules: {
            transport: [ TCP ],
            streamMuxer: [ Mplex ],
            connEncryption: [ NOISE ],
            // we add the DHT module that will enable Peer and Content Routing
            dht: KadDHT
        },
        config: {
            dht: {
                // dht must be enabled
                enabled: true
            },
            peerDiscovery: {
                bootstrap: {
                    interval: 60e3,
                    enabled: true,
                    list: [
                        '/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ',
                        '/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN',
                        '/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb',
                        '/dnsaddr/bootstrap.libp2p.io/p2p/QmZa1sAxajnQjVM8WjWXoMbmPd7NsWhfKsPkErzpm9wGkp',
                        '/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa',
                        '/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt'
                    ],
                }
            }
        }
    })

    // node.connectionManager.on('peer:connect', (connection) => {
    //     console.log('Connection established to:', connection.remotePeer.toB58String())	// Emitted when a new connection has been created
    // })
      
    // node.on('peer:discovery', (peerId) => {
    //     // No need to dial, autoDial is on
    //     console.log('Discovered:', peerId.toB58String())
    // })
      
    await node.start()
    return node
}
