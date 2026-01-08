# Changelog

## [0.0.13](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.12...v0.0.13) (2026-01-08)


### Features

* add `ReadStorage`/`PreimageSource` impls for `Box<T>` ([#43](https://github.com/matter-labs/zksync-os-interface/issues/43)) ([b355798](https://github.com/matter-labs/zksync-os-interface/commit/b355798ddca587c2093d7d38a05df7092943d5af))
* add support for TxValidator alongside Tracer ([#35](https://github.com/matter-labs/zksync-os-interface/issues/35)) ([bdb2a98](https://github.com/matter-labs/zksync-os-interface/commit/bdb2a98981a67abf59253cf73a4a25c177bd9173))


### Bug Fixes

* "computaional_native_used" typo ([#42](https://github.com/matter-labs/zksync-os-interface/issues/42)) ([0685c97](https://github.com/matter-labs/zksync-os-interface/commit/0685c97913462670c8c837629a8153a9757e4aef))

## [0.0.12](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.11...v0.0.12) (2025-12-19)


### Features

* add code_size_limit to BlockContext ([#36](https://github.com/matter-labs/zksync-os-interface/issues/36)) ([52143fb](https://github.com/matter-labs/zksync-os-interface/commit/52143fba7b72e6ef2e10faaa464464d490614b04))

## [0.0.11](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.10...v0.0.11) (2025-12-19)


### Features

* pretty print RLP encoded tx in debug ([#37](https://github.com/matter-labs/zksync-os-interface/issues/37)) ([deab246](https://github.com/matter-labs/zksync-os-interface/commit/deab246cc81dc08d62e1220bb0cc4c22c0d8bdf5))

## [0.0.10](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.9...v0.0.10) (2025-11-13)


### Features

* add 'stack' to tracing interface ([#32](https://github.com/matter-labs/zksync-os-interface/issues/32)) ([1c9afae](https://github.com/matter-labs/zksync-os-interface/commit/1c9afae82b3a1197bac991bea772394b8c8ff0f8))

## [0.0.9](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.8...v0.0.9) (2025-10-31)


### Features

* Changes for 4844 support ([#30](https://github.com/matter-labs/zksync-os-interface/issues/30)) ([ec7cb2b](https://github.com/matter-labs/zksync-os-interface/commit/ec7cb2b97bb384efc1296b9b026bcb056be81494))

## [0.0.8](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.7...v0.0.8) (2025-10-16)


### Features

* support both rlp and abi encoded txs ([#28](https://github.com/matter-labs/zksync-os-interface/issues/28)) ([6d5957d](https://github.com/matter-labs/zksync-os-interface/commit/6d5957dfdfde1701f5adf3f732aef40a60634582))

## [0.0.7](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.6...v0.0.7) (2025-10-07)


### Bug Fixes

* Fix simulate tx trait ([#23](https://github.com/matter-labs/zksync-os-interface/issues/23)) ([d17bd1c](https://github.com/matter-labs/zksync-os-interface/commit/d17bd1c048c832cd3b497abf32f274b705cad1b7))
* Replace gas_per_pubdata with pubdata_price ([#22](https://github.com/matter-labs/zksync-os-interface/issues/22)) ([e40086a](https://github.com/matter-labs/zksync-os-interface/commit/e40086a1623e49de08e26317ce73320b1c4f91bf))

## [0.0.6](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.5...v0.0.6) (2025-10-06)


### Features

* support for EVM tracing ([#20](https://github.com/matter-labs/zksync-os-interface/issues/20)) ([301d6e0](https://github.com/matter-labs/zksync-os-interface/commit/301d6e08b412466f6b3e8c50de6073a48079086f))

## [0.0.5](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.4...v0.0.5) (2025-09-17)


### Features

* rename protocol version ([#18](https://github.com/matter-labs/zksync-os-interface/issues/18)) ([a803e72](https://github.com/matter-labs/zksync-os-interface/commit/a803e72331f44164d95dcb64be93065b19e582f0))

## [0.0.4](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.3...v0.0.4) (2025-09-12)


### Features

* add crate description ([#16](https://github.com/matter-labs/zksync-os-interface/issues/16)) ([1957e28](https://github.com/matter-labs/zksync-os-interface/commit/1957e280de6f28ac38e77401d1e7ed4398c261a5))

## [0.0.3](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.2...v0.0.3) (2025-09-12)


### Features

* publish crate ([#14](https://github.com/matter-labs/zksync-os-interface/issues/14)) ([8a3e417](https://github.com/matter-labs/zksync-os-interface/commit/8a3e4175538b5ca8f6a24a553ae0f7f5deea5f16))

## [0.0.2](https://github.com/matter-labs/zksync-os-interface/compare/v0.0.1...v0.0.2) (2025-09-11)


### Features

* Add `Config` associated type ([#10](https://github.com/matter-labs/zksync-os-interface/issues/10)) ([ae890ee](https://github.com/matter-labs/zksync-os-interface/commit/ae890eec99ff0763d78db40f8e4f0fac76ac4554))
* add account diffs ([#8](https://github.com/matter-labs/zksync-os-interface/issues/8)) ([358361a](https://github.com/matter-labs/zksync-os-interface/commit/358361a576793c81e8fc670e99b9c9d39bb89d38))
* add Pectra errors ([#5](https://github.com/matter-labs/zksync-os-interface/issues/5)) ([c4d8101](https://github.com/matter-labs/zksync-os-interface/commit/c4d8101f20a58cbb5dae2c81625d0371de8f8b03))
* add SimulateTx trait ([#6](https://github.com/matter-labs/zksync-os-interface/issues/6)) ([719b559](https://github.com/matter-labs/zksync-os-interface/commit/719b55960cabe240bda6e0da38af280f93ddc5c8))
* add support for `no_std` mode ([#12](https://github.com/matter-labs/zksync-os-interface/issues/12)) ([03974f6](https://github.com/matter-labs/zksync-os-interface/commit/03974f6aab1b26dea340133069949860afcbb668))
* get rid of features; preimage-agnostic ([#13](https://github.com/matter-labs/zksync-os-interface/issues/13)) ([04aa514](https://github.com/matter-labs/zksync-os-interface/commit/04aa514f167a543255732bc6d99e5705334347e5))
* initial implementation ([#1](https://github.com/matter-labs/zksync-os-interface/issues/1)) ([aa784b3](https://github.com/matter-labs/zksync-os-interface/commit/aa784b3e7d99b1f1df36c211eda4dfccd81fe671))
* make `RunBlock`/`SimulateTx` dynamic of self ([#7](https://github.com/matter-labs/zksync-os-interface/issues/7)) ([deb46da](https://github.com/matter-labs/zksync-os-interface/commit/deb46dad854b9f226e309d07652a7f429e614460))
