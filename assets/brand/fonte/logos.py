"""Generate vector logos (text converted to outlines) for homedeck."""
import io, os, sys
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.pens.boundsPen import BoundsPen
import uharfbuzz as hb

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = sys.argv[1]
os.makedirs(OUT, exist_ok=True)

f = TTFont(os.path.join(HERE, 'outfit-latin-500-normal.woff2'))
f.flavor = None
buf = io.BytesIO(); f.save(buf); data = buf.getvalue()
f = TTFont(io.BytesIO(data))
upem = f['head'].unitsPerEm
gs = f.getGlyphSet()
order = f.getGlyphOrder()

def word_path(text, size, tracking_em=-0.027):
    face = hb.Face(data); font = hb.Font(face)
    b = hb.Buffer(); b.add_str(text); b.guess_segment_properties()
    hb.shape(font, b, {"kern": True, "liga": True})
    sc = size / upem
    pen = SVGPathPen(gs)
    bp = BoundsPen(gs)
    x = 0
    for info, pos in zip(b.glyph_infos, b.glyph_positions):
        g = order[info.codepoint]
        t = (sc, 0, 0, -sc, x + pos.x_offset * sc, -pos.y_offset * sc)
        gs[g].draw(TransformPen(pen, t)); gs[g].draw(TransformPen(bp, t))
        x += pos.x_advance * sc + tracking_em * size
    return pen.getCommands(), bp.bounds  # bounds in y-down coords (baseline at 0)

MARK = '''<path d="M{a} {b} L{c} {d} L{e} {b}" fill="none" stroke="{col}" stroke-width="58" stroke-linecap="round" stroke-linejoin="round"/>
<circle cx="{a}" cy="{b}" r="66" fill="{col}"/><circle cx="{c}" cy="{d}" r="66" fill="{col}"/><circle cx="{e}" cy="{b}" r="66" fill="{col}"/>
<rect x="{rx}" y="{ry}" width="374" height="110" rx="55" fill="{col}"/>'''

def mark(col, ox=0, oy=0):
    # original geometry, bbox x243..837, y255..783 -> shift to 0,0
    return '<g transform="translate({:.2f} {:.2f})">'.format(ox - 243, oy - 255) + MARK.format(
        a=309, b=541, c=540, d=321, e=771, rx=353, ry=673, col=col) + '</g>'

MW, MH = 594, 528

def svg(w, h, body, title='homedeck'):
    return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w:.0f} {h:.0f}" width="{w:.0f}" height="{h:.0f}" role="img" aria-label="{title}"><title>{title}</title>{body}</svg>\n'

files = {}
for name, col in (('white', '#FFFFFF'), ('black', '#0C0C0E')):
    # symbol
    pad = 0
    files[f'homedeck-simbolo-{name}.svg'] = svg(MW, MH, mark(col))
    # wordmark
    size = 525
    d, (x0, y0, x1, y1) = word_path('homedeck', size)
    ww, wh = x1 - x0, y1 - y0
    files[f'homedeck-wordmark-{name}.svg'] = svg(ww, wh, f'<path transform="translate({-x0:.2f} {-y0:.2f})" fill="{col}" d="{d}"/>')
    # horizontal lockup
    gap = 148
    H = max(MH, wh)
    my = (H - MH) / 2; wy = (H - wh) / 2
    body = mark(col, 0, my) + f'<path transform="translate({MW + gap - x0:.2f} {wy - y0:.2f})" fill="{col}" d="{d}"/>'
    files[f'homedeck-logo-horizontal-{name}.svg'] = svg(MW + gap + ww, H, body)
    # stacked lockup
    size2 = 300
    d2, (a0, b0, a1, b1) = word_path('homedeck', size2)
    w2, h2 = a1 - a0, b1 - b0
    W = max(MW, w2); gap2 = 120
    body = mark(col, (W - MW) / 2, 0) + f'<path transform="translate({(W - w2) / 2 - a0:.2f} {MH + gap2 - b0:.2f})" fill="{col}" d="{d2}"/>'
    files[f'homedeck-logo-vertical-{name}.svg'] = svg(W, MH + gap2 + h2, body)

for k, v in files.items():
    open(os.path.join(OUT, k), 'w').write(v)
print('\n'.join(files))
