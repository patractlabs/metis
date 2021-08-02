# Metis Examples

## Build

At first we need build the contracts, note the examples may call each other, so it need build all contracts.

```bash
yarn .
yarn build
```

If build is success, it will be artifacts in `example/artifacts`:

```bash
 ll ./artifacts  
total 2.5M
-rw-rw-r-- 1 user user  92K 7月  30 17:59 access_control.contract
-rw-rw-r-- 1 user user  23K 7月  30 17:59 access_control.json

...
```

## Run Tests

To run tests, we need run a europa node for envirment:

```bash
europa --tmp --dev
```

To install the europa, can look at [europa](https://github.com/patractlabs/europa), Note the params must be `--tmp --dev`, some testcases will depend on the init Token for test accounts.

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

Use `yarn test` may error by too mush testcases.
