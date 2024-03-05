import { Devnet, DevnetConfig } from 'alpaca-addon';
import { expect } from 'chai';

type AccountData = {
    account_address: string;
    public_key: string;
    private_key: string;
    balance: string;
};

function dataFeed(val: string): void {
    console.log('hehe', val);
}

describe('Alpaca-addon', function () {
    it('Start devnet', async function () {
        let config: DevnetConfig = {
            seed: 20,
            port: 5050,
            totalAccounts: 2,
        };

        let res = await Devnet.start(config, dataFeed);
        let accountData: AccountData[] = res.map((el) => el as unknown as AccountData);
        expect(accountData[0]).to.deep.equal({
            account_address: '0x1c12570d28567eeb3b5323dc3cc6ccb9a094b809c8e616c8a13f34c258736c8',
            public_key: '0x2ccb5e686566bc0b13d6c48bffb9cd442f1e2b34aab6e3b287a30d0c9c5432e',
            private_key: '0x24c2f9272b152d6b18d447962d2cfe07',
            balance: '0x3635c9adc5dea00000',
        });

        expect(accountData[1]).to.deep.equal({
            account_address: '0x7b9731f027bd4bea71ff5def1a9f41e8d1d3b9d522fe1d0c07cdd48c46d153c',
            public_key: '0x89481151008c9f6ec7b235328c446af7ce86b3b4228c51f02447e22836e0b0',
            private_key: '0x644188b5afe811fe97dbc7ee8393047a',
            balance: '0x3635c9adc5dea00000',
        });
    });

    it('Same endpoint error', async function () {
        let config: DevnetConfig = {
            seed: 20,
            port: 5050,
            totalAccounts: 2,
        };

        try {
            // TODO: make shutdown/stop method on Devnet
            await Devnet.start(config, dataFeed);
            expect.fail('Should of received an error');
        } catch (err: any) {
            expect(err.message).to.eq('error creating server listener: Address already in use (os error 48)');
            expect(err.type).to.eq(1);
            expect(err.tag).to.eq('alpaca-addon');
        }
    });
});
