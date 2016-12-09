const protolevel = {
    createPositions() {
        const positions = {};
        for (let y = 0; y < HEIGHT; y++) {
            for (let x = Math.floor((HEIGHT - y) / 2); x < WIDTH - Math.floor(y / 2); x++) {
                positions[xy2pos(x, y)] = true;
            }
        }
        return positions;
    },


    createInnerPositions() {
        const innerPositions = {};
        for (let y = 1; y < HEIGHT - 1; y++) {
            for (let x = Math.floor((HEIGHT - y) / 2) + 1; x < WIDTH - Math.floor(y / 2) - 1; x++) {
                innerPositions[xy2pos(x, y)] = true;
            }
        }
        return innerPositions;
    },


    createPassable() {
        const passable = {};
        for (pos in this.positions) {
            passable[pos] = false;
        }
        passable[this.startpos] = true;
        return passable;
    },


    carveCaves() {
        shuffle(Object.keys(this.innerPositions)).forEach(pos => {
            if (countGroups(Number(pos), pos => this.passable[pos]) !== 1) {
                this.passable[pos] = true;
            }
        });
    },
};


function Level(startpos) {
    const level = Object.create(protolevel);
    level.startpos = startpos;
    level.positions = level.createPositions();
    level.innerPositions = level.createInnerPositions();
    level.passable = level.createPassable();
    level.carveCaves();
    return level;
}


const lvl = Level(xy2pos(24, 16)).passable;
