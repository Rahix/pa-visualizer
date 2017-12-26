from sfml import sf
import time

settings = sf.ContextSettings()
settings.antialiasing_level = 8

window = sf.RenderWindow(sf.VideoMode(800, 600, 32),
                                    "PAVisualizer",
                                    sf.Style.DEFAULT,
                                    settings)

window.view.center = (0, 0)
window.view.size = (2.2, 2.2)

rect = sf.RectangleShape()

width = 0.8 / COLUMNS

def frame(left, right):
    window.clear(sf.Color(50,50,50))

    for i in range(0, COLUMNS):
        size = (left[i] + right[i]) / 2

        rect.position = (i / COLUMNS * 2 - 1, -size/2)
        rect.size = (width, size)

        window.draw(rect)

    window.display()

    for ev in window.events:
        if ev.type == sf.Event.CLOSED or (ev.type == sf.Event.KEY_RELEASED and ev["code"] == sf.Keyboard.ESCAPE):
            window.close()
            return False

    # time.sleep(0.02)
    return True
