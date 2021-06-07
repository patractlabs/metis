import os
import unittest
import logging

from substrateinterface import SubstrateInterface, ContractMetadata, Keypair
from substrateinterface.utils.ss58 import ss58_encode
from patractinterface.contract import ContractAPI, ContractFactory
from patractinterface.unittest.env import SubstrateTestEnv

example_path = "../../../../../examples/erc20-ownable/contracts/target/ink/"

default_gas_limit = 200000000000

class ERC20TestCase(unittest.TestCase):
    @classmethod
    def tearDown(cls):
        cls.env.stop_node()

    @classmethod
    def setUpClass(cls):
        logging.info("init deplay erc20")

        cls.env = SubstrateTestEnv.create_europa(port=39944)
        cls.env.start_node()
        cls.substrate=SubstrateInterface(url=cls.env.url(), type_registry_preset=cls.env.typ(), type_registry=cls.env.types())

        cls.contract_metadata = ContractMetadata.create_from_file(
            metadata_file=os.path.join(os.path.dirname(__file__), example_path, 'metadata.json'),
            substrate=cls.substrate
        )

        cls.factory = ContractFactory.create_from_file(
            substrate=cls.substrate, 
            code_file=os.path.join(os.path.dirname(__file__), example_path, 'erc20ownable.wasm'),
            metadata_file=os.path.join(os.path.dirname(__file__), example_path, 'metadata.json')
        )

        cls.alice = Keypair.create_from_uri('//Alice')
        cls.bob = Keypair.create_from_uri('//Bob')

        cls.api = cls.factory.new(cls.alice, 1000000 * (10 ** 15), endowment=10**15, gas_limit=1000000000000)

    def transfer(self):
        supply = self.api.total_supply(self.alice)
        self.assertEqual(supply, 1000000 * (10 ** 15))

        res = self.api.transfer(self.alice, self.bob.ss58_address, 10000, gas_limit=default_gas_limit)
        self.assertTrue(res.is_success)
        self.check_balance_of(self.bob.ss58_address, 10000)

    def transfer_from(self):
        res = self.api.transfer_from(self.alice,
            self.alice.ss58_address, 
            self.bob.ss58_address, 
            10000, gas_limit=default_gas_limit)
        self.assertTrue(res.is_success)

    def approve(self):
        res = self.api.approve(self.alice, self.bob.ss58_address, 10000, gas_limit=default_gas_limit)
        self.assertTrue(res.is_success)
        allowance = self.api.allowance(self.alice, self.alice.ss58_address, self.bob.ss58_address)
        self.assertEqual(allowance, 10000)

    def check_balance_of(self, acc, value):
        res = self.api.balance_of(self.alice, acc)
        self.assertEqual(res, value)

    def metadata_info(self):
        name = self.api.name(self.alice)
        self.assertEqual(name, 'MetisTestToken')

        symbol = self.api.symbol(self.alice)
        self.assertEqual(symbol, 'MET')

        decimals = self.api.decimals(self.alice)
        self.assertEqual(decimals, 18)

        logging.info(f'metadata_info {name} {symbol} {decimals}')


    def test_exec_and_read(self):
        self.transfer()
        self.approve()
        self.transfer_from()
        self.metadata_info()

if __name__ == '__main__':
    unittest.main()
