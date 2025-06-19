const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function transferFunds() {
    console.log('💰 开始给测试账户充值...');
    
    try {
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });
        
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');
        const charlie = keyring.addFromUri('//Charlie');
        const dave = keyring.addFromUri('//Dave');
        const eve = keyring.addFromUri('//Eve');
        
        const amount = 10n * 1000000000000000000n; // 10 DOT
        
        console.log('正在给Bob充值...');
        await new Promise((resolve, reject) => {
            api.tx.balances.transferKeepAlive(bob.address, amount).signAndSend(alice, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ Bob 充值成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Bob 充值失败'));
                }
            });
        });
        
        console.log('正在给Charlie充值...');
        await new Promise((resolve, reject) => {
            api.tx.balances.transferKeepAlive(charlie.address, amount).signAndSend(alice, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ Charlie 充值成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Charlie 充值失败'));
                }
            });
        });
        
        console.log('正在给Dave充值...');
        await new Promise((resolve, reject) => {
            api.tx.balances.transferKeepAlive(dave.address, amount).signAndSend(alice, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ Dave 充值成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Dave 充值失败'));
                }
            });
        });
        
        console.log('正在给Eve充值...');
        await new Promise((resolve, reject) => {
            api.tx.balances.transferKeepAlive(eve.address, amount).signAndSend(alice, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ Eve 充值成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Eve 充值失败'));
                }
            });
        });
        
        console.log('💰 所有账户充值完成！');
        
    } catch (error) {
        console.error('❌ 充值失败:', error);
    }
    
    process.exit(0);
}

transferFunds().catch(console.error); 