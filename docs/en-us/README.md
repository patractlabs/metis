# The Document of Metis

[Metis](https://github.com/patractlabs/metis) will be the Wasm contract standard library developed by Patract Labs. Patract Labs will work with community forces to formulate various Wasm contract standards, develop corresponding implementations of ink! and Ask! versions, and conduct audits by professional institutions.

Metis proposed and implemented MCCI architecture. MCCI architecture facilitates smart contract development by composititing independent components. Here are a list of components currently implemented:

- ERC20 and its expansion components
- ERC721 and its expansion components
- ERC777 and its expansion components
- ERC1155 and its expansion components
- Ownable
- AccessControl
- TimelockController
- Escrow(PullPayment)
- support(ERC165)
- ReentrancyGuard
- Pauseable

## Roadmap

- ~~**[M1]** Implement basic macros and implementations for components; improve component testing support; developers can build regular DAPPs based on Metis~~
- **[M2]** Complete component macros; complete component development support so that developers can build custom components; complete the api support corresponding to the metis component.
- **[M3]** Rich component library; complete component and API support for governance and financial mechanism; thorough mathematical library for contract development to support DeFi-type contracts that require complex calculations.
- **[MR]** According to the ink!'s iterative progress, community feedback, contract upgrades, contract proxy and cross-contract call support, refactor Event-related implementations, improve basic components and add development assistance macros to reduce duplication while ensuring auditability Code.

## Usage of Document

the documentation of the Metis based on `docsify`

```bash
sudo npm i docsify-cli -g
```

to make the documentation, can use `docsify serve` command:

```bash
cd docs
docsify serve .
```

then can read in `http://localhost:3000`
