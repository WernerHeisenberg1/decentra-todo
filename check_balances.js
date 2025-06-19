const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function checkBalances() {
    try {
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });
        
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');
        const charlie = keyring.addFromUri('//Charlie');
        const dave = keyring.addFromUri('//Dave');
        const eve = keyring.addFromUri('//Eve');
        
        const accounts = { alice, bob, charlie, dave, eve };
        
        console.log('💰 账户余额查询:');
        
        for (const [name, account] of Object.entries(accounts)) {
            const balance = await api.query.system.account(account.address);
            console.log(`  ${name}: ${balance.data.free.toHuman()}`);
        }
        
    } catch (error) {
        console.error('❌ 查询失败:', error);
    }
    
    process.exit(0);
}

checkBalances(); 