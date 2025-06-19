const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function setBalances() {
    console.log('💰 使用sudo设置账户余额...');
    
    try {
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });
        
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice'); // sudo user
        const charlie = keyring.addFromUri('//Charlie');
        const dave = keyring.addFromUri('//Dave');
        const eve = keyring.addFromUri('//Eve');
        
        const balance = 100n * 1000000000000000000n; // 100 DOT
        
        console.log('正在给Charlie设置余额...');
        await new Promise((resolve, reject) => {
            api.tx.sudo.sudo(
                api.tx.balances.forceSetBalance(charlie.address, balance)
            ).signAndSend(alice, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ Charlie 余额设置成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Charlie 余额设置失败'));
                }
            });
        });
        
        console.log('正在给Dave设置余额...');
        await new Promise((resolve, reject) => {
            api.tx.sudo.sudo(
                api.tx.balances.forceSetBalance(dave.address, balance)
            ).signAndSend(alice, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ Dave 余额设置成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Dave 余额设置失败'));
                }
            });
        });
        
        console.log('正在给Eve设置余额...');
        await new Promise((resolve, reject) => {
            api.tx.sudo.sudo(
                api.tx.balances.forceSetBalance(eve.address, balance)
            ).signAndSend(alice, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ Eve 余额设置成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Eve 余额设置失败'));
                }
            });
        });
        
        console.log('💰 所有账户余额设置完成！');
        
    } catch (error) {
        console.error('❌ 设置失败:', error);
    }
    
    process.exit(0);
}

setBalances().catch(console.error); 