// generate automatically
pub const BIOME_DATA: [(&'static str, f32, f32, u32); 256] = [
    // (name, temperature, rainfall, water_color) 
    (/*   0 */ "Ocean"                             , 0.50, 0.50, 0x3F76E4),
    (/*   1 */ "Plains"                            , 0.80, 0.40, 0x3F76E4),
    (/*   2 */ "Desert"                            , 2.00, 0.00, 0x3F76E4),
    (/*   3 */ "Mountains"                         , 0.20, 0.30, 0x3F76E4),
    (/*   4 */ "Forest"                            , 0.70, 0.80, 0x3F76E4),
    (/*   5 */ "Taiga"                             , 0.25, 0.80, 0x3F76E4),
    (/*   6 */ "Swamp"                             , 0.80, 0.90, 0x617B64),
    (/*   7 */ "River"                             , 0.50, 0.50, 0x3F76E4),
    (/*   8 */ "Nether"                            , 2.00, 0.00, 0x3F76E4),
    (/*   9 */ "The End"                           , 0.50, 0.50, 0x3F76E4),
    (/*  10 */ "Frozen Ocean"                      , 0.00, 0.50, 0x3938C9),
    (/*  11 */ "Frozen River"                      , 0.00, 0.50, 0x3938C9),
    (/*  12 */ "Snowy Tundra"                      , 0.00, 0.50, 0x3F76E4),
    (/*  13 */ "Snowy Mountains"                   , 0.00, 0.50, 0x3F76E4),
    (/*  14 */ "Mushroom Fields"                   , 0.90, 1.00, 0x3F76E4),
    (/*  15 */ "Mushroom Field Shore"              , 0.90, 1.00, 0x3F76E4),
    (/*  16 */ "Beach"                             , 0.80, 0.40, 0x3F76E4),
    (/*  17 */ "Desert Hills"                      , 2.00, 0.00, 0x3F76E4),
    (/*  18 */ "Wooded Hills"                      , 0.70, 0.80, 0x3F76E4),
    (/*  19 */ "Taiga Hills"                       , 0.25, 0.80, 0x3F76E4),
    (/*  20 */ "Mountain Edge"                     , 0.20, 0.30, 0x3F76E4),
    (/*  21 */ "Jungle"                            , 0.95, 0.90, 0x3F76E4),
    (/*  22 */ "Jungle Hills"                      , 0.95, 0.90, 0x3F76E4),
    (/*  23 */ "Jungle Edge"                       , 0.95, 0.80, 0x3F76E4),
    (/*  24 */ "Deep Ocean"                        , 0.50, 0.50, 0x3F76E4),
    (/*  25 */ "Stone Shore"                       , 0.20, 0.30, 0x3F76E4),
    (/*  26 */ "Snowy Beach"                       , 0.05, 0.30, 0x3F76E4),
    (/*  27 */ "Birch Forest"                      , 0.60, 0.60, 0x3F76E4),
    (/*  28 */ "Birch Forest Hills"                , 0.60, 0.60, 0x3F76E4),
    (/*  29 */ "Dark Forest"                       , 0.70, 0.80, 0x3F76E4),
    (/*  30 */ "Snowy Taiga"                       , -0.50, 0.40, 0x3F76E4),
    (/*  31 */ "Snowy Taiga Hills"                 , -0.50, 0.40, 0x3F76E4),
    (/*  32 */ "Giant Tree Taiga"                  , 0.30, 0.80, 0x3F76E4),
    (/*  33 */ "Giant Tree Taiga Hills"            , 0.30, 0.80, 0x3F76E4),
    (/*  34 */ "Wooded Mountains"                  , 0.20, 0.30, 0x3F76E4),
    (/*  35 */ "Savanna"                           , 1.20, 0.00, 0x3F76E4),
    (/*  36 */ "Savanna Plateau"                   , 1.00, 0.00, 0x3F76E4),
    (/*  37 */ "Badlands"                          , 2.00, 0.00, 0x3F76E4),
    (/*  38 */ "Wooded Badlands Plateau"           , 2.00, 0.00, 0x3F76E4),
    (/*  39 */ "Badlands Plateau"                  , 2.00, 0.00, 0x3F76E4),
    (/*  40 */ "Small End Islands"                 , 0.50, 0.50, 0x3F76E4),
    (/*  41 */ "End Midlands"                      , 0.50, 0.50, 0x3F76E4),
    (/*  42 */ "End Highlands"                     , 0.50, 0.50, 0x3F76E4),
    (/*  43 */ "End Barrens"                       , 0.50, 0.50, 0x3F76E4),
    (/*  44 */ "Warm Ocean"                        , 0.80, 0.50, 0x43D5EE),
    (/*  45 */ "Lukewarm Ocean"                    , 0.80, 0.50, 0x45ADF2),
    (/*  46 */ "Cold Ocean"                        , 0.80, 0.50, 0x3D57D6),
    (/*  47 */ "Deep Warm Ocean"                   , 0.80, 0.50, 0x3F76E4),
    (/*  48 */ "Deep Lukewarm Ocean"               , 0.80, 0.50, 0x3F76E4),
    (/*  49 */ "Deep Cold Ocean"                   , 0.80, 0.50, 0x3F76E4),
    (/*  50 */ "Deep Frozen Ocean"                 , 0.80, 0.50, 0x3F76E4),
    (/*  51 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  52 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  53 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  54 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  55 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  56 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  57 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  58 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  59 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  60 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  61 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  62 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  63 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  64 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  65 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  66 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  67 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  68 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  69 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  70 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  71 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  72 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  73 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  74 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  75 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  76 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  77 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  78 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  79 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  80 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  81 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  82 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  83 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  84 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  85 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  86 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  87 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  88 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  89 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  90 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  91 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  92 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  93 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  94 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  95 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  96 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  97 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  98 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/*  99 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 100 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 101 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 102 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 103 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 104 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 105 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 106 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 107 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 108 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 109 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 110 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 111 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 112 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 113 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 114 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 115 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 116 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 117 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 118 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 119 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 120 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 121 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 122 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 123 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 124 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 125 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 126 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 127 */ "The Void"                          , 0.50, 0.50, 0x3F76E4),
    (/* 128 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 129 */ "Sunflower Plains"                  , 0.80, 0.40, 0x3F76E4),
    (/* 130 */ "Desert Lakes"                      , 2.00, 0.00, 0x3F76E4),
    (/* 131 */ "Gravelly Mountains"                , 0.20, 0.30, 0x3F76E4),
    (/* 132 */ "Flower Forest"                     , 0.70, 0.80, 0x3F76E4),
    (/* 133 */ "Taiga Mountains"                   , 0.25, 0.80, 0x3F76E4),
    (/* 134 */ "Swamp Hills"                       , 0.80, 0.90, 0x3F76E4),
    (/* 135 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 136 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 137 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 138 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 139 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 140 */ "Ice Spikes"                        , 0.00, 0.50, 0x3F76E4),
    (/* 141 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 142 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 143 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 144 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 145 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 146 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 147 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 148 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 149 */ "Modified Jungle"                   , 0.95, 0.90, 0x3F76E4),
    (/* 150 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 151 */ "Modified Jungle Edge"              , 0.95, 0.80, 0x3F76E4),
    (/* 152 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 153 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 154 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 155 */ "Tall Birch Forest"                 , 0.60, 0.60, 0x3F76E4),
    (/* 156 */ "Tall Birch Hills"                  , 0.60, 0.60, 0x3F76E4),
    (/* 157 */ "Dark Forest Hills"                 , 0.70, 0.80, 0x3F76E4),
    (/* 158 */ "Snowy Taiga Mountains"             , -0.50, 0.40, 0x3F76E4),
    (/* 159 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 160 */ "Giant Spruce Taiga"                , 0.25, 0.80, 0x3F76E4),
    (/* 161 */ "Giant Spruce Taiga Hills"          , 0.25, 0.80, 0x3F76E4),
    (/* 162 */ "Gravelly Mountains+"               , 0.20, 0.30, 0x3F76E4),
    (/* 163 */ "Shattered Savanna"                 , 1.10, 0.00, 0x3F76E4),
    (/* 164 */ "Shattered Savanna Plateau"         , 1.00, 0.00, 0x3F76E4),
    (/* 165 */ "Eroded Badlands"                   , 2.00, 0.00, 0x3F76E4),
    (/* 166 */ "Modified Wooded Badlands Plateau"  , 2.00, 0.00, 0x3F76E4),
    (/* 167 */ "Modified Badlands Plateau"         , 2.00, 0.00, 0x3F76E4),
    (/* 168 */ "Bamboo Jungle"                     , 0.95, 0.90, 0x3F76E4),
    (/* 168 */ "Bamboo Jungle Hills"               , 0.95, 0.90, 0x3F76E4),
    (/* 170 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 171 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 172 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 173 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 174 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 175 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 176 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 177 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 178 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 179 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 180 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 181 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 182 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 183 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 184 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 185 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 186 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 187 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 188 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 189 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 190 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 191 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 192 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 193 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 194 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 195 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 196 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 197 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 198 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 199 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 200 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 201 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 202 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 203 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 204 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 205 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 206 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 207 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 208 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 209 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 210 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 211 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 212 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 213 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 214 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 215 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 216 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 217 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 218 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 219 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 220 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 221 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 222 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 223 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 224 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 225 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 226 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 227 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 228 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 229 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 230 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 231 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 232 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 233 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 234 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 235 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 236 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 237 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 238 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 239 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 240 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 241 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 242 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 243 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 244 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 245 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 246 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 247 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 248 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 249 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 250 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 251 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 252 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 253 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 254 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
    (/* 255 */ "Unknown Biome"                     , 0.80, 0.40, 0x3F76E4),
];

pub const COLORMAP_GRASS: &'static [u8] = include_bytes!("grass.png");

pub const COLORMAP_FOLIAGE: &'static [u8] = include_bytes!("foliage.png");

