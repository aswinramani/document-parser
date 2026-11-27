window.BENCHMARK_DATA = {
  "lastUpdate": 1795803105568,
  "repoUrl": "https://github.com/aswinramani/document-parser",
  "entries": {
    "Parse benchmarks (ubuntu-latest runner)": [
      {
        "commit": {
          "author": {
            "email": "aswinramani5@gmail.com",
            "name": "Aswin Ramani",
            "username": "aswinramani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f43966025acbaff1148eb79cf5fece84e62aeb10",
          "message": "day34: update benchmark worflow (#50)",
          "timestamp": "2026-11-09T00:46:59+05:30",
          "tree_id": "1674a7ee5d8925562b932551670ed986932ca375",
          "url": "https://github.com/aswinramani/document-parser/commit/f43966025acbaff1148eb79cf5fece84e62aeb10"
        },
        "date": 1794165632366,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse/normal_100kb/path",
            "value": 350823,
            "range": "± 1652",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_100kb/memory",
            "value": 327165,
            "range": "± 1759",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/path",
            "value": 3296076,
            "range": "± 27470",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/memory",
            "value": 3056208,
            "range": "± 21959",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/path",
            "value": 16908812,
            "range": "± 313152",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/memory",
            "value": 15484990,
            "range": "± 269927",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/path",
            "value": 34995266,
            "range": "± 636379",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/memory",
            "value": 32267385,
            "range": "± 937945",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/path",
            "value": 92027599,
            "range": "± 1019670",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/memory",
            "value": 85548652,
            "range": "± 828004",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/path",
            "value": 187239846,
            "range": "± 3073337",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/memory",
            "value": 174152414,
            "range": "± 1448186",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aswinramani5@gmail.com",
            "name": "Aswin Ramani",
            "username": "aswinramani"
          },
          "committer": {
            "email": "aswinramani5@gmail.com",
            "name": "Aswin Ramani",
            "username": "aswinramani"
          },
          "distinct": true,
          "id": "e50af0c1ae63eaeacf5c2a6d90586d2f102a91aa",
          "message": "day35: update readme",
          "timestamp": "2026-11-09T19:09:06+05:30",
          "tree_id": "e985296aecce34af53a3299a4b303aade7560e65",
          "url": "https://github.com/aswinramani/document-parser/commit/e50af0c1ae63eaeacf5c2a6d90586d2f102a91aa"
        },
        "date": 1794231765578,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse/normal_100kb/path",
            "value": 333396,
            "range": "± 2385",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_100kb/memory",
            "value": 325601,
            "range": "± 2236",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/path",
            "value": 3209389,
            "range": "± 55503",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/memory",
            "value": 3018456,
            "range": "± 17028",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/path",
            "value": 15822608,
            "range": "± 553719",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/memory",
            "value": 14946825,
            "range": "± 41896",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/path",
            "value": 31715577,
            "range": "± 190890",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/memory",
            "value": 29899738,
            "range": "± 1249373",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/path",
            "value": 83959441,
            "range": "± 1167319",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/memory",
            "value": 77409962,
            "range": "± 4081688",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/path",
            "value": 177066145,
            "range": "± 835329",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/memory",
            "value": 165640865,
            "range": "± 3131301",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aswinramani5@gmail.com",
            "name": "Aswin Ramani",
            "username": "aswinramani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f019cad002ba2a2ce4cbf7159ee77d9c4108b240",
          "message": "add agent rules (#51)\n\n* day36: add AGENTS.md and CLAUDE.md agent rules\n\n* day36: add expected FHIR Condition fixture and serde_json dev-dependency",
          "timestamp": "2026-11-27T22:57:15+05:30",
          "tree_id": "bbd64c3c79be33cebfdff623324302da7f1e4f82",
          "url": "https://github.com/aswinramani/document-parser/commit/f019cad002ba2a2ce4cbf7159ee77d9c4108b240"
        },
        "date": 1795800648101,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse/normal_100kb/path",
            "value": 299364,
            "range": "± 1979",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_100kb/memory",
            "value": 298293,
            "range": "± 1117",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/path",
            "value": 2956180,
            "range": "± 103299",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/memory",
            "value": 2788693,
            "range": "± 15365",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/path",
            "value": 14703549,
            "range": "± 76565",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/memory",
            "value": 13767083,
            "range": "± 63763",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/path",
            "value": 30108754,
            "range": "± 336579",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/memory",
            "value": 28407013,
            "range": "± 670628",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/path",
            "value": 81207701,
            "range": "± 408467",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/memory",
            "value": 81191302,
            "range": "± 841912",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/path",
            "value": 163618371,
            "range": "± 747472",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/memory",
            "value": 164008097,
            "range": "± 1463455",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "31f938971382387c5b485d142a1ed4902da07682",
          "message": "build(deps): bump github/codeql-action from 4.39.1 to 4.39.3 (#48)\n\nBumps [github/codeql-action](https://github.com/github/codeql-action) from 4.39.1 to 4.39.3.\n- [Release notes](https://github.com/github/codeql-action/releases)\n- [Changelog](https://github.com/github/codeql-action/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/github/codeql-action/compare/v4.39.1...v4.39.3)\n\n---\nupdated-dependencies:\n- dependency-name: github/codeql-action\n  dependency-version: 4.39.3\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-11-27T23:03:02+05:30",
          "tree_id": "ace120b925cb1e83595938dfe5f16888f98e40ea",
          "url": "https://github.com/aswinramani/document-parser/commit/31f938971382387c5b485d142a1ed4902da07682"
        },
        "date": 1795801001379,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse/normal_100kb/path",
            "value": 354289,
            "range": "± 1676",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_100kb/memory",
            "value": 319169,
            "range": "± 3078",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/path",
            "value": 3323490,
            "range": "± 31024",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/memory",
            "value": 3053765,
            "range": "± 16255",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/path",
            "value": 16963197,
            "range": "± 294285",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/memory",
            "value": 15523380,
            "range": "± 210811",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/path",
            "value": 36625685,
            "range": "± 891949",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/memory",
            "value": 33192470,
            "range": "± 630486",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/path",
            "value": 93303811,
            "range": "± 3071051",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/memory",
            "value": 85735954,
            "range": "± 822807",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/path",
            "value": 187476873,
            "range": "± 4893609",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/memory",
            "value": 171813758,
            "range": "± 2699184",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aswinramani5@gmail.com",
            "name": "Aswin Ramani",
            "username": "aswinramani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1e0537b0b23c248cd0310ff03a04ec2bb2e79aef",
          "message": "day36: init mcp-server workspace skeleton (#52)\n\n* day36: init mcp-server workspace skeleton\n\n* day36: document MCP server in AGENTS.md and register it in .mcp.json\n\n* day36: run tests and clippy across the whole workspace",
          "timestamp": "2026-11-27T23:38:01+05:30",
          "tree_id": "12fc8c86589ce5c7c2bd55296a3bece86a2ae2b0",
          "url": "https://github.com/aswinramani/document-parser/commit/1e0537b0b23c248cd0310ff03a04ec2bb2e79aef"
        },
        "date": 1795803105155,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse/normal_100kb/path",
            "value": 387724,
            "range": "± 3230",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_100kb/memory",
            "value": 382278,
            "range": "± 4235",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/path",
            "value": 3734866,
            "range": "± 19681",
            "unit": "ns/iter"
          },
          {
            "name": "parse/normal_1mb/memory",
            "value": 3508387,
            "range": "± 21745",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/path",
            "value": 18689076,
            "range": "± 389328",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_5mb/memory",
            "value": 17604114,
            "range": "± 115351",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/path",
            "value": 39108664,
            "range": "± 1613046",
            "unit": "ns/iter"
          },
          {
            "name": "parse/medium_10mb/memory",
            "value": 36636063,
            "range": "± 654420",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/path",
            "value": 102748138,
            "range": "± 1201349",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_25mb/memory",
            "value": 95054652,
            "range": "± 2607448",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/path",
            "value": 205561312,
            "range": "± 3656531",
            "unit": "ns/iter"
          },
          {
            "name": "parse/big_50mb/memory",
            "value": 190339088,
            "range": "± 1032581",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}