import os
from brownie import CrowdSale, DELF, accounts, network, config


def main():
    dev = accounts.load('matic')
    delf = DELF.deploy('DELFINE', 'DELF', 18, 1000000000, {'from': dev})
    # tokenwallet = accounts[9]
    # print(f'token wallet is {tokenwallet}')
    # print(f'dev wallet is {dev}')
    # delf.transfer(dev, (1000000000 * 10 ** 18) / 2, {'from': dev})
    crowdsale = CrowdSale.deploy(
        1000,
        dev,
        delf.address,
        dev,
        {'from': dev})

    delf.approve(crowdsale.address, 1 * 10 ** 9 * 10 ** 18, {'from': dev})
    allowance = delf.allowance(dev, crowdsale)
    print(f'The allowance is {allowance / 10 ** 18}')
    print(f'Remaining tokens are: {crowdsale.getRemainingTokens()}')

    # accounts[2].transfer(
    #     '0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266', 9 * 10 ** 18)
    # print('Transfered 9th eth to Account 2')

    return delf, crowdsale, dev
