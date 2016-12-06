from random import Random

from actor import Actor, Player
from level import Level
from schedule import Schedule


class Game:
    
    def __init__(self, seed):
        self.seed = seed

        self.random = Random()
        self.random.seed(seed)

        self.output = None
        self.outputqueue = []
        self.level = None
        self.schedule = None

        self.player = Player(self)

        self.descend()


    def descend(self):
        newlevel = Level()
        newschedule = Schedule()
        self.level = newlevel
        self.schedule = newschedule


    def input(self, line):
        """Receive input and send output"""
        command, blank, arg = line.partition(' ')

        delay = self.player.act(command, arg)
        id = self.schedule.pushpop(self.player.id, delay)
        while id:
            if id == self.player.id:
                break
            actor = Actor.actors[id]
            delay = actor.act()
            id = self.schedule.pushpop(id, delay)
        self.output('\n'.join(self.outputqueue))


    def queueoutput(self, line):
        self.outputqueue.append(str(self.schedule.time) + ' ' + line)