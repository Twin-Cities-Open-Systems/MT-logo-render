# Duople example recipes

Per Spencer, 2026-08-18: this tool's actual reason for existing --
using color/shape to denote a claim's verification state (HEE's
Thesis -> Duople distinction, see `human-execution-engine`'s
`README.md#thesis-vs-duople`). Three states, three simple recipes:

- `thesis.yaml` -- gray circle, dot mark: unverified, not yet checked
- `duople.yaml` -- gold/cyan triangle, check mark: verified, confirmed
  by more than one independent source (see
  `library/py/hee_hash/soa.py`'s `duople=a:b` -- same concept, this
  tool's visual expression of it)
- `miss.yaml` -- red square, x mark: checked, and wrong -- a real,
  scored miss, not silently dropped

Render any of them once the CLI contract fixes in this branch land:

    logo-render render --file examples/duople/duople.yaml --targets png,ansi

Known limitation, not fixed here: `triangle` shape doesn't support
`fill: split:N` -- degrades to solid with a warning
("Unsupported split count for shape"). `duople.yaml` uses `split:3`
to match the concept's three-edge structure but currently renders
solid. Real gap if triangle's 3-way structure should ever be visible
in the fill itself, not just implied by shape choice.
