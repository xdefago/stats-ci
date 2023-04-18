# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.0.2 (2023-04-18)

<csr-id-cbe5378cc3a1d90ccd0b94902088090ef208635b/>
<csr-id-8e896dc892de9a663d6dbcca215537656b151455/>
<csr-id-53ee3be8395defd3ef72cdace35a6c2f08c3c56f/>
<csr-id-beaf51a1c53c260d097712189a76c6f630b6f6af/>
<csr-id-b4108b440e56d09c3e67e4babb2fec881c121f0f/>

### Documentation

 - <csr-id-8e5d0797e912774b13bc43c0f5c2339add8dbfb8/> add items to todo list
 - <csr-id-c289c9127935d12ff59a93e80c93cffac46a5a94/> delete entry on index calculations (fixed)
 - <csr-id-146690eef241c3f9e22f351fdd6d997759d3ab42/> comment issue with index calculations
 - <csr-id-9b8b0189b258c927b7e4c21940a36ebf213b2f88/> reorder tags by colors in README
 - <csr-id-06759624d2555953b7f6881fb745774659075cf5/> add a TODO list
 - <csr-id-4c2b9059e03263fde597f78a3d40df0dced4fd17/> fix typos in comment of example

### New Features

 - <csr-id-c728c69d3213481a115076f72c9e3aaa92bf5161/> add function for wilson from success rate
 - <csr-id-5ebc5ec4cb7cee85ed0c4239df1fbbb38445f4c0/> export calculation of indices for quantiles

### Bug Fixes

 - <csr-id-2ef6b3ca4312e2cecd5ff266b9ef51440a212622/> check bounds without panic for quantile::ci
 - <csr-id-4435b0ce8c3f7898897c294c6aba460df29cbe5b/> conversion error reporting wrong type

### Other

 - <csr-id-cbe5378cc3a1d90ccd0b94902088090ef208635b/> bump version number to 0.0.2

### Refactor

 - <csr-id-8e896dc892de9a663d6dbcca215537656b151455/> reformat code
 - <csr-id-53ee3be8395defd3ef72cdace35a6c2f08c3c56f/> code formatting

### Style

 - <csr-id-beaf51a1c53c260d097712189a76c6f630b6f6af/> add trailing comma
 - <csr-id-b4108b440e56d09c3e67e4babb2fec881c121f0f/> rename local variable z_2 to z_sq

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 23 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 15 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release stats-ci v0.0.2 ([`bfd80f6`](https://github.com/xdefago/stats-ci/commit/bfd80f61aed6518e5c10e4c80ef98c900e5cc92e))
    - Add trailing comma ([`beaf51a`](https://github.com/xdefago/stats-ci/commit/beaf51a1c53c260d097712189a76c6f630b6f6af))
    - Add function for wilson from success rate ([`c728c69`](https://github.com/xdefago/stats-ci/commit/c728c69d3213481a115076f72c9e3aaa92bf5161))
    - Rename local variable z_2 to z_sq ([`b4108b4`](https://github.com/xdefago/stats-ci/commit/b4108b440e56d09c3e67e4babb2fec881c121f0f))
    - Bump version number to 0.0.2 ([`cbe5378`](https://github.com/xdefago/stats-ci/commit/cbe5378cc3a1d90ccd0b94902088090ef208635b))
    - Export calculation of indices for quantiles ([`5ebc5ec`](https://github.com/xdefago/stats-ci/commit/5ebc5ec4cb7cee85ed0c4239df1fbbb38445f4c0))
    - Add items to todo list ([`8e5d079`](https://github.com/xdefago/stats-ci/commit/8e5d0797e912774b13bc43c0f5c2339add8dbfb8))
    - Reformat code ([`8e896dc`](https://github.com/xdefago/stats-ci/commit/8e896dc892de9a663d6dbcca215537656b151455))
    - Delete entry on index calculations (fixed) ([`c289c91`](https://github.com/xdefago/stats-ci/commit/c289c9127935d12ff59a93e80c93cffac46a5a94))
    - Code formatting ([`53ee3be`](https://github.com/xdefago/stats-ci/commit/53ee3be8395defd3ef72cdace35a6c2f08c3c56f))
    - Check bounds without panic for quantile::ci ([`2ef6b3c`](https://github.com/xdefago/stats-ci/commit/2ef6b3ca4312e2cecd5ff266b9ef51440a212622))
    - Comment issue with index calculations ([`146690e`](https://github.com/xdefago/stats-ci/commit/146690eef241c3f9e22f351fdd6d997759d3ab42))
    - Reorder tags by colors in README ([`9b8b018`](https://github.com/xdefago/stats-ci/commit/9b8b0189b258c927b7e4c21940a36ebf213b2f88))
    - Add a TODO list ([`0675962`](https://github.com/xdefago/stats-ci/commit/06759624d2555953b7f6881fb745774659075cf5))
    - Fix typos in comment of example ([`4c2b905`](https://github.com/xdefago/stats-ci/commit/4c2b9059e03263fde597f78a3d40df0dced4fd17))
    - Conversion error reporting wrong type ([`4435b0c`](https://github.com/xdefago/stats-ci/commit/4435b0ce8c3f7898897c294c6aba460df29cbe5b))
    - State that mean CIs are computed with the t-value ([`c70c0bf`](https://github.com/xdefago/stats-ci/commit/c70c0bff19d4d7c0f315386d09bf113a0c97bd79))
    - Return None if the interval falls ourside the data ([`da24755`](https://github.com/xdefago/stats-ci/commit/da24755c0679acedb88ac5993c2641be85de32aa))
    - Disable default serde feature; document in readme ([`98f0d22`](https://github.com/xdefago/stats-ci/commit/98f0d22b8b787841a4367c91ef2e930ad92c61bf))
    - Reorder and tag badges ([`09ad8e0`](https://github.com/xdefago/stats-ci/commit/09ad8e02c4d189f50b4d6c0fa263bb0257487cb3))
    - Add badges ([`3a711b9`](https://github.com/xdefago/stats-ci/commit/3a711b9cf2af658324423aaae76290ec5e7a1926))
    - Rename build action ([`ac0168f`](https://github.com/xdefago/stats-ci/commit/ac0168ff6b9f85ae58c94d5eecb7e28cbe3e8f55))
    - Add documentation link ([`1ee83f3`](https://github.com/xdefago/stats-ci/commit/1ee83f3cbcecd5b675fee64d699c3b40066f6452))
</details>

## v0.0.1 (2023-04-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 56 commits contributed to the release over the course of 8 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix license badges ([`d8c760a`](https://github.com/xdefago/stats-ci/commit/d8c760a86bb1174578a2c9d2b33fed422434059a))
    - Change license to MIT or APACHE ([`aa25582`](https://github.com/xdefago/stats-ci/commit/aa25582cfb9dd3aa26a2ec9a98ecefc7b33bfc92))
    - Add explanations for the types of CIs ([`2a097e7`](https://github.com/xdefago/stats-ci/commit/2a097e7485dc534ca678747af003a1b9873542da))
    - Add an example to measure runtime (casually) ([`2c43ef2`](https://github.com/xdefago/stats-ci/commit/2c43ef2476d152913df2e0c8015284c22f77631c))
    - Remove unused import ([`450075b`](https://github.com/xdefago/stats-ci/commit/450075b196e75080a6fe5574bea695d2208c52f2))
    - Replace iterator with for-loop ([`d1ecda1`](https://github.com/xdefago/stats-ci/commit/d1ecda11f74ac09183f1b36ce3c67c9ed489fe02))
    - Expand rust doc at the root ([`7813b24`](https://github.com/xdefago/stats-ci/commit/7813b244f372ddaddecf13082b28d83032a28618))
    - Implement kahan summation ([`b8001cd`](https://github.com/xdefago/stats-ci/commit/b8001cd3478804ad84362ad913faca3dde56268d))
    - Add automatic link ([`67820a3`](https://github.com/xdefago/stats-ci/commit/67820a3bf1e297c0f13e79ad48d1969b0231ab6e))
    - Fix doc test indentation ([`f6ae245`](https://github.com/xdefago/stats-ci/commit/f6ae245d99794a09bd58540b085d8239c11b513e))
    - Fix comment ([`2f4f3f5`](https://github.com/xdefago/stats-ci/commit/2f4f3f5ffe9979fadf97a0fa99ae9f87d9979cec))
    - Move z_value and t_value to mod stats ([`0ad8f7e`](https://github.com/xdefago/stats-ci/commit/0ad8f7e192f9de73769e22a41b4eda22b47c5c22))
    - Fix significance test for Wilson; add reference ([`2a4ec66`](https://github.com/xdefago/stats-ci/commit/2a4ec664e67060081f6df4c1fcd01f0955134cd3))
    - Rephrase and add references ([`2e0fdcb`](https://github.com/xdefago/stats-ci/commit/2e0fdcb4607ce97e322d3a39b0e9b3903a3cd696))
    - Add references to crate doc ([`838151a`](https://github.com/xdefago/stats-ci/commit/838151a30c0f7e4f14c5309bb4b62f5372fcb0e0))
    - Merge branch 'main' of https://github.com/xdefago/stats-ci ([`7fb5913`](https://github.com/xdefago/stats-ci/commit/7fb5913bcd5b81dd2d78a7f7866f8cfedd22f377))
    - Reformat code ([`fe28eab`](https://github.com/xdefago/stats-ci/commit/fe28eab9a356f1496d992af4a6e57d01037adfe4))
    - Add github commit action ([`c6e1462`](https://github.com/xdefago/stats-ci/commit/c6e14622db3b2c65c0fce789452e7bb94fc7a6b7))
    - Make t_value public; rephrase comment ([`337b704`](https://github.com/xdefago/stats-ci/commit/337b704c91b1456177959a41d1c05a13cfd18106))
    - Rephrase description comment (remove two-sided) ([`312d1cd`](https://github.com/xdefago/stats-ci/commit/312d1cd733214136b8d39947ad5146ff81b844df))
    - Add motivation and examples ([`1e119a3`](https://github.com/xdefago/stats-ci/commit/1e119a37a9d61f51f663154a0c7c2996e5ce6786))
    - Add harmonic and geometric test cases ([`fbcac6e`](https://github.com/xdefago/stats-ci/commit/fbcac6e4e5230c004ca079967db45ac8b9fcb58a))
    - Change var name in tests: interval -> ci ([`9d97002`](https://github.com/xdefago/stats-ci/commit/9d97002f4c388ccba674f19e8a8b8e6d9377ce33))
    - Fix Display as closed intervals ([`6ffe377`](https://github.com/xdefago/stats-ci/commit/6ffe377cc7f690ddb62e8c2ead9412eebb62abf3))
    - Represent confidence with dedicated enum ([`576ac5c`](https://github.com/xdefago/stats-ci/commit/576ac5c25aa1efa0dc401975c7cfbcb1d5f4cc79))
    - Remove unused z_value_two_sided() ([`f3082ed`](https://github.com/xdefago/stats-ci/commit/f3082edf7ed5ff63861f1ea252abbc226f45253a))
    - Make one/two sided explicit ([`93fcf63`](https://github.com/xdefago/stats-ci/commit/93fcf63039d3585e917c2ee79e46d71a6a3b81d5))
    - Reformat code ([`f3ec93d`](https://github.com/xdefago/stats-ci/commit/f3ec93db1677f528827c03fc417f97c43ea7fa5e))
    - Reduce use of unwrap in doctests ([`5e91e10`](https://github.com/xdefago/stats-ci/commit/5e91e109c9194e2de19c4e7954268cf74ef2d2f9))
    - Add error variant from String ([`a980d4e`](https://github.com/xdefago/stats-ci/commit/a980d4eb5709c7ad1b346c7b8f8a0b5430266109))
    - Restructure mean module; add harmonic/geometric ([`9d0cd80`](https://github.com/xdefago/stats-ci/commit/9d0cd807a169e72ea689db13a6246086f5deafa6))
    - Add crate meta information and forbid unsafe_code ([`ee824bd`](https://github.com/xdefago/stats-ci/commit/ee824bdf38051cad8fff8315bc6a3cd0f847b28f))
    - Add root comments to crate ([`0342ddf`](https://github.com/xdefago/stats-ci/commit/0342ddf25ad0ccbf736563afdb5ae595be80bd43))
    - Add conversion errors ([`aba1e67`](https://github.com/xdefago/stats-ci/commit/aba1e67b66da8eb2115228562d53c184c2ed1b74))
    - Add comparison for Interval (partial order) ([`ac84f1d`](https://github.com/xdefago/stats-ci/commit/ac84f1d56d3c3b3c160176954fc1bfc0787991b5))
    - Add serde feature; add categories ([`d98384e`](https://github.com/xdefago/stats-ci/commit/d98384e293ff39ef19a097f818bd0263c4a66cb2))
    - Add metadata about the crate ([`f6b8c45`](https://github.com/xdefago/stats-ci/commit/f6b8c45ae8b2251490b15f78d80a6ec8076edc8b))
    - Initialize normal distribution statically ([`73bb4d7`](https://github.com/xdefago/stats-ci/commit/73bb4d7b762e4bb00aae86b9679b6f3b3d61ee03))
    - Add comments; rewrite tests ([`3b28b57`](https://github.com/xdefago/stats-ci/commit/3b28b5706d113e184fcbbdeb1cbec6ffe3e31bac))
    - Remove empty file ([`4e60111`](https://github.com/xdefago/stats-ci/commit/4e601112e3dc65b7029d0aed714c03712a03fd3e))
    - Reorganize doc comments ([`49baf69`](https://github.com/xdefago/stats-ci/commit/49baf69b1c98141dd2708f3b2df25b7bb524aae0))
    - Reformat code ([`43b5fa3`](https://github.com/xdefago/stats-ci/commit/43b5fa3178bb1d210a54673a8de20862566c2e65))
    - Reformat code ([`5534493`](https://github.com/xdefago/stats-ci/commit/5534493755a33e46db649f822735ab525a30b182))
    - Reorganize intervals ([`d357a72`](https://github.com/xdefago/stats-ci/commit/d357a729810877dcd7e4bb486fbdd4b85b02a780))
    - Remove useless clone(); simplify match code ([`8c15a0a`](https://github.com/xdefago/stats-ci/commit/8c15a0ae84a3742103ff7f91532282df59718e77))
    - Change to Wilson score intervals ([`f74e975`](https://github.com/xdefago/stats-ci/commit/f74e975faa73401fb9e0df670f7036dfddb42846))
    - Add disclaimer ([`f3172d8`](https://github.com/xdefago/stats-ci/commit/f3172d8b499bad9e481bc9a5dcd92eb0abcf0fe0))
    - Add one- vs. two-sided ([`c424656`](https://github.com/xdefago/stats-ci/commit/c4246562c392a96b9ea8a10f28f5d5c5cf4e24b5))
    - Add FloatConversion ([`7ac6886`](https://github.com/xdefago/stats-ci/commit/7ac68865348ee14ab59de266e53c6a7161afbd69))
    - Clarify description of mean.ci() ([`8740fa2`](https://github.com/xdefago/stats-ci/commit/8740fa2ff6eaf5a047be71ab310da8b062207177))
    - Add test of confidence level ([`25a25c2`](https://github.com/xdefago/stats-ci/commit/25a25c292d2dfac336c1afa0120b1a758cfbf810))
    - Reformat code ([`9c92aa8`](https://github.com/xdefago/stats-ci/commit/9c92aa808f239fbfe397e6d92a590037cfa6c25e))
    - Fix comments ([`f93a8ae`](https://github.com/xdefago/stats-ci/commit/f93a8ae203b38d6156217c2aa30fe3f22efe218f))
    - Refactor intervals ([`3cb0bb9`](https://github.com/xdefago/stats-ci/commit/3cb0bb95fa0715ee4b540e51e971dcb1477c4960))
    - Fix version number to 0.0.1 ([`98f4d3f`](https://github.com/xdefago/stats-ci/commit/98f4d3f150c342cd7a77271b5a535fe3e74031f5))
    - Initial commit ([`0f7eda5`](https://github.com/xdefago/stats-ci/commit/0f7eda528888d811f21d74c4c1b9b6f972e56e39))
</details>

