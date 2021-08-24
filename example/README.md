# Metis Examples

## Build

Metis examples is built by [Redspot](https://redspot.patract.io/).

At first, we need to construct redspot environment, then compile all example contracts. Note the examples contracts may call each other, so it needs to build all contracts.

```bash
# re-construct redspot environment
yarn .
# compile example contracts
npx redspot compile
```

If build is success, the compiled contracts are all in `example/artifacts`:

```bash
 ll ./artifacts  
total 2.5M
-rw-rw-r-- 1 user user  92K 7月  30 17:59 access_control.contract
-rw-rw-r-- 1 user user  23K 7月  30 17:59 access_control.json
# ...
```

## Run Tests

To run tests, we need run a blockchain node with `pallet-contracts` for environment. We advise you to use [Europa](https://github.com/patractlabs/europa) as the contract platform node:

To install the europa, please refer to [europa](https://github.com/patractlabs/europa) Readme.

Note the starting params should be `--tmp`, some testcases will depend on the init Token for test accounts.

```bash
europa --tmp
```

As the testcases is large, so we need run tests for each contract:

```bash
npx redspot test ./tests/token/erc721.test.ts
npx redspot test ./tests/token/erc721.burnable.test.ts
npx redspot test ./tests/token/erc721.enumerable.test.ts
npx redspot test ./tests/token/erc721.pausable.test.ts
npx redspot test ./tests/token/erc721.urlstorage.test.ts
npx redspot test ./tests/token/erc777.test.ts
npx redspot test ./tests/token/erc1155.test.ts
npx redspot test ./tests/governance/timelock_controller.test.ts
npx redspot test ./tests/security/reentrancy-guard.test.ts
npx redspot test ./tests/access-control/access-control.test.ts
npx redspot test ./tests/access-control/ownable.test.ts
npx redspot test ./tests/tools/erc165.test.ts
npx redspot test ./tests/utils/escrow.test.ts
```

If you run `yarn test` or `npx redspot test` directly, it may take a long time for so many testcases, and parallel processing may cause some error in substrate transaction pool then abort the test.
