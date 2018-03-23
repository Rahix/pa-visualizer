#!/usr/bin/python3
""" Visualizer written in python """

from sfml import sf

SETTINGS = sf.ContextSettings()
SETTINGS.antialiasing_level = 8

WINDOW = sf.RenderWindow(sf.VideoMode(800, 600, 32),
                         "PAVisualizer",
                         sf.Style.DEFAULT,
                         SETTINGS)

WINDOW.view.center = (0, 0)
WINDOW.view.size = (2.2, 2.2)

RECT = sf.RectangleShape()

WIDTH = 0.8 / COLUMNS

def frame(left, right):
    """ Render a single frame """
    WINDOW.clear(sf.Color(50, 50, 50))

    for i in range(0, COLUMNS):
        size = (left[i] + right[i]) / 2

        RECT.position = (i / COLUMNS * 2 - 1, -size/2)
        RECT.size = (WIDTH, size)

        WINDOW.draw(RECT)

    WINDOW.display()

    for event in WINDOW.events:
        if event.type == sf.Event.CLOSED \
           or (event.type == sf.Event.KEY_RELEASED \
           and event["code"] == sf.Keyboard.ESCAPE):
            WINDOW.close()
            return False

    return True
