import struct
from collections import defaultdict

seen = defaultdict(int)

MOVEMENT = struct.Struct('<BBBBBff')

# small world
WORLD_START_X = 656
WORLD_CENTER_X = 33590
WORLD_END_X = 66508

WORLD_START_Y = 656
WORLD_SURFACE_Y = 5334
WORLD_END_Y = 18486
FT_POS_SCALE = 8

HELD_ITEM_INVENTORY_INDEX = 58

def handle_1(tag, body):
    print('protocol magic:', body.decode('ascii'))

def handle_4(tag, body):
    name_len = struct.unpack('<B', body[2:3])[0]
    name = body[3:3 + name_len].decode('ascii')
    print('player name:', name)

def handle_5(tag, body):
    # inventory
    index, count, a, item_id = struct.unpack('<HHBH', body)
    # a = 72 for accessories
    # a = 73 or 76 for vanity accessories
    # a = 0 otherwise
    """
    if index == HELD_ITEM_INVENTORY_INDEX:
        print(f'holding {count} items with id {item_id}')
    else:
        print(f'inventory: {count} items with id {item_id} at pos {index}, a {a}')
    """

def handle_8(tag, body):
    # during login no idea what
    assert all(x == 0xff for x in body)

def handle_12(tag, body):
    x, y, timer, how = struct.unpack('<HHIB', body)
    # x, y seem to always be ffff
    # how = 0 for death
    # how = 2 for recall or mirror
    print('back to spawn', x, y, timer, how)

def handle_13(tag, body):
    flags, speed_flag, c, d, hotbar, x, y = MOVEMENT.unpack(body[:MOVEMENT.size])
    """
    if flags & 0b0000_0001:
        print('holding up')
    if flags & 0b0000_0010:
        print('holding down')
    if flags & 0b0000_0100:
        print('holding left')
    if flags & 0b0000_1000:
        print('holding right')
    if flags & 0b0001_0000:
        print('holding space')
    if flags & 0b0010_0000:
        print('holding mouse')
    if flags & 0b0100_0000:
        print('facing right')
    """

    assert not (flags & 0b1000_0000)
    assert (speed_flag & 0b1111_1011) == 0b0001_0000
    assert c in (0, 0b00000010)  # seems 0 for void bag
    assert d == 0b00000000

    # normalize
    nx = round((x - WORLD_CENTER_X) / FT_POS_SCALE, 2)
    ny = round((WORLD_SURFACE_Y - y) / FT_POS_SCALE, 2)

    if speed_flag & 0b0000_0100:
        dx, dy = struct.unpack('<ff', body[MOVEMENT.size:])
        dx = round(dx, 2)
        dy = round(dy, 2)
    else:
        dx = 0.0
        dy = 0.0

    #print('item', hotbar, 'move', nx, ny, '( + speed', dx, dy, ')')

def handle_16(tag, body):
    life, max_life = struct.unpack('<HH', body)

def handle_17(tag, body):
    # placing or removing (tile_id 0) things from the world
    x, y, tile_id, d = struct.unpack('<HHHB', body)
    assert d == 0

def handle_21(tag, body):
    # item moved in the world
    created, x, y, dx, dy, count, modifier, d, item_id = struct.unpack('<?ffffHBBH', body)
    assert d == 0

def handle_22(tag, body):
    a, b = struct.unpack('<BB', body)
    # seems to always be 00ff or ff00

def handle_27(tag, body):
    # shooting 007ba9ff4600c88f45369b60414429c9be000e003029009a994940
    a, x, y, dx, dy, player_id, kind, flags = struct.unpack('<BffffBHB', body[:21])
    body = body[21:]
    assert a == 0
    ai = flags & 0b0000_1111
    while ai:
        ai >>= 1
        x = struct.unpack('<f', body[:4])[0]
        body = body[4:]
        # no idea what this means but it is a float

    if flags & 0b0001_0000:
        damage = struct.unpack('<H', body[:2])[0]
        body = body[2:]
    if flags & 0b0010_0000:
        knockback = struct.unpack('<f', body[:4])[0]
        body = body[4:]
    if flags & 0b0100_0000:
        original_damage = struct.unpack('<H', body[:2])[0]
        body = body[2:]
    if flags & 0b1000_0000:
        projectile_id = struct.unpack('<H', body[:2])[0]
        body = body[2:]

    assert not body

def handle_28(tag, body):
    # damaging enemies
    a, damage, maybe_knockback, d, e = struct.unpack('<BHfBB', body)
    assert a == 0

def handle_29(tag, body):
    # seems to be 0000 while shooting
    assert body == b'\0\0'

def handle_36(tag, body):
    # changing biome triggers this, may be flags
    zones = struct.unpack('<4B', body)

def handle_40(tag, body):
    # -1 if not talking to anybody
    talk_npc = struct.unpack('<h', body)[0]

def handle_41(tag, body):
    # projectile animation, rotation in radians (0 left, clockwise)
    rotation, stage = struct.unpack('<fH', body)

def handle_42(tag, body):
    mana, max_mana = struct.unpack('<HH', body)

def handle_50(tag, body):
    mount_id = body[6]
    print('using mount', mount_id)

def handle_53(tag, body):
    # on melee kills
    a, b, c = struct.unpack('<BHH', body)
    assert a == 0

def handle_56(tag, body):
    # after login
    assert body == b'\0'

def handle_59(tag, body):
    # occurs taking damage sometimes
    a, b = struct.unpack('<BH', body)

def handle_68(tag, body):
    print('player uuid4:', body.decode('ascii'))

def handle_73(tag, body):
    assert not body
    print('teleported to beach')

def handle_82(tag, body):
    # after login but no idea what
    pass

def handle_84(tag, body):
    stealth = struct.unpack('<f', body)[0]

def handle_109(tag, body):
    # wires
    direction, x, y, end_y, wire_flag = struct.unpack('<?HHHB', body)
    _red_wire = wire_flag & 0b0000_0001
    _green_wire = wire_flag & 0b0000_0010
    _blue_wire = wire_flag & 0b0000_0100
    _yellow_wire = wire_flag & 0b0000_1000
    _actuator_wire = wire_flag & 0b0001_0000
    _remove_wire = wire_flag & 0b0010_0000

def handle_117(tag, body):
    # no idea on this one, happens when taking damage
    pass
"""
new 55 (1): ab0005000000
new 28 (1): 00a000000070410000
new 28 (2): 009400000070410000
new 28 (1): 00640133336f410201

new 59 (1): 071b01
new 53 (1): 0018006801

"""


def handle_new(tag, body):
    print(f'new {tag} ({seen[tag]}): {body.hex()}')
    pass  # so we can comment out the print easily

def handle(packet):
    tag = packet[2]
    seen[tag] += 1

    if tag in (6, 138):
        return  # these don't have more info

    _player_index = packet[3]
    body = packet[4:]

    (globals().get(f'handle_{tag}') or handle_new)(tag, body)
