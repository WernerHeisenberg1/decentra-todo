const { ApiPromise, WsProvider } = require('@polkadot/api');

async function checkMethods() {
  try {
    const api = await ApiPromise.create({ provider: new WsProvider('ws://127.0.0.1:9944') });
    console.log('Balances pallet methods:');
    console.log(Object.keys(api.tx.balances));
    
    console.log('\nAvailable pallets:');
    console.log(Object.keys(api.tx));
    
    process.exit(0);
  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  }
}

checkMethods(); 