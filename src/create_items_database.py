import sqlite3

from sqlite3 import Error


def create_connection(db_file):
    """ create a database connection to a SQLite database """
    try:
        connection = sqlite3.connect(db_file)
        return connection
    except Error as e:
        print(e)


def create_table(con):
    cursor_object = con.cursor()
    cursor_object.execute('CREATE TABLE items')


def insert_item(con, item):

    sql = ''' INSERT INTO items 
    (entry,class, subclass, name, displayid, Quality, Flags, BuyCount,
    BuyPrice, SellPrice, InventoryType, AllowableClass, AllowableRace, ItemLevel, RequiredLevel,
    RequiredSkill, RequiredSkillRank, requiredspell, requiredhonorrank, RequiredCityRank,
    RequiredReputationFaction, RequiredReputationRank, maxcount, stackable, ContainerSlots,
    stat_type1, stat_value1, stat_type2, stat_value2, stat_type3, stat_value3, stat_type4,
    stat_value4, stat_type5, stat_value5, stat_type6, stat_value6, stat_type7, stat_value7,
    stat_type8, stat_value8, stat_type9, stat_value9, stat_type10, stat_value10, dmg_min1,
    dmg_max1, dmg_type1, dmg_min2, dmg_max2, dmg_type2, dmg_min3, dmg_max3, dmg_type3, dmg_min4,
    dmg_max4, dmg_type4, dmg_min5, dmg_max5, dmg_type5, armor, holy_res, fire_res, nature_res,
    frost_res, shadow_res, arcane_res, delay, ammo_type, RangedModRange, spellid_1, spelltrigger_1,
    spellcharges_1, spellppmRate_1, spellcooldown_1, spellcategory_1, spellcategorycooldown_1, spellid_2,
    spelltrigger_2, spellcharges_2, spellppmRate_2, spellcooldown_2, spellcategory_2,
    spellcategorycooldown_2, spellid_3, spelltrigger_3, spellcharges_3, spellppmRate_3,
    spellcooldown_3, spellcategory_3, spellcategorycooldown_3, spellid_4, spelltrigger_4,
    spellcharges_4, spellppmRate_4, spellcooldown_4, spellcategory_4, spellcategorycooldown_4,
    spellid_5, spelltrigger_5, spellcharges_5, spellppmRate_5, spellcooldown_5, spellcategory_5,
    spellcategorycooldown_5, bonding, description, PageText, LanguageID, PageMaterial, startquest,
    lockid, Material, sheath, RandomProperty, block, itemset, MaxDurability, area, Map, BagFamily,
    ScriptName, DisenchantID, FoodType, minMoneyLoot, maxMoneyLoot, Duration, ExtraFlags)
    VALUES
    (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,
    ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,
    ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
    '''

    cur = con.cursor()
    cur.execute(sql, item)

    return cur.lastrowid


if __name__ == '__main__':

    items = []
    with open('items_to_insert', 'r') as f:
        for cnt, line in enumerate(f):
            items.append(line.rstrip())
            if cnt == 10:
                break
    print(items)













