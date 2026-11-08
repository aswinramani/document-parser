window.BENCHMARK_DATA = {
  "lastUpdate": 1794165632604,
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
      }
    ]
  }
}