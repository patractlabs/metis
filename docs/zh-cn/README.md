# Metis开发文档

[Metis](https://github.com/patractlabs/metis) 是 Patract 开发的 WebAssembly 合约标准库。Patract Labs 将联合社区力量一起制定各项 WebAssembly 合约标准，开发 ink! 和 Ask! 版本的对应实现，并进行专业机构审计。

Metis提出并实现了`MCCI 架构`。`MCCI 架构`通过组合独立组件来促进了智能合约的开发。 以下是当前实现的组件列表：

- ERC20及其扩展组件
- ERC721及其扩展组件
- ERC777及其扩展组件
- ERC1155及其扩展组件
- Ownable
- AccessControl
- TimelockController
- PullPayment
- support(ERC165)
- ReentrancyGuard
- Pauseable

## 路线图

- ~~**[M1]** 实现组件的基本宏和实现； 改进组件测试支持； 开发者可以基于 Metis 构建常规 DAPP 。~~
- **[M2]** 完整的组件宏； 完善的组件开发支持，以便开发人员能够建立自定义组件； 完成 Metis 组件对应的 API 支持。
- **[M3]**** 丰富的组件库； 对治理和财务机制的完整组件和 API 支持； 用于合约开发的完整数学库，以支持需要复杂计算的 DeFi 类型合约。
- **[MR]** 根据基于ink!的迭代进度及社区反馈，合约升级, 合约代理与跨合约调用支持, 重构Event相关实现， 完善基础组件并添加开发辅助宏, 在保证可审计性的情况下减少重复代码。

## 文档使用说明

Metis文档基于`docsify`, 可以基于npm安装:

```bash
sudo npm i docsify-cli -g
```

如果网速较慢， 可以使用`cnpm`

获取docs, 并启动:

```bash
cd docs
docsify serve .
```

默认运行在 [http://localhost:3000](http://localhost:3000) 上，直接可以浏览。
