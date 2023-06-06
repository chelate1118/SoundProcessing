package sound.ui

import sound.Processing
import sound.ui.filter.FilterGraph
import java.util.*

class UI(private val sketch: Processing) {
    private val uiComponents: TreeSet<Component> = TreeSet()
    private val tempRemove = ArrayList<Component>()
    private val tempCreate = ArrayList<Component>()
    private var mouseComponent: Component? = null
    private var mousePressedComponent: Component? = null

    fun setup() {
        createComponent(Background(sketch))
        createComponent(FilterGraph(sketch))
    }

    fun draw() {
        applyAllTemp()
        updateMouseComponent()
        updateAllComponent()
    }

    fun mousePressed() {
        mousePressedComponent = mouseComponent
        mousePressedComponent?.mouseClicked()
    }

    fun mouseReleased() {
        mousePressedComponent = null
    }

    private fun updateAllComponent() {
        for (component in uiComponents) {
            val state = component.update()

            if (state == State.Kill) {
                tempRemove.add(component)
            }

            component.display()
        }
    }

    private fun updateMouseComponent() {
        val prevMouseComponent = mouseComponent
        mouseComponent = findMouseComponent()

        if (prevMouseComponent != mouseComponent) {
            prevMouseComponent?.mouseOut()
            mouseComponent?.mouseIn()
        }

        mousePressedComponent?.mousePressed()
    }

    private fun findMouseComponent(): Component? {
        val reverseIterator = uiComponents.descendingIterator()

        while (reverseIterator.hasNext()) {
            val component = reverseIterator.next()
            if (component.isMouseIn(sketch.mousePosition)) {
                return component
            }
        }
        return null
    }

    private fun applyAllTemp() {
        removeAllTemp()
        createAllTemp()
    }

    private fun removeAllTemp() {
        for (component in tempRemove) {
            component.onDestroy()
            uiComponents.remove(component)
        }
        tempRemove.clear()
    }

    private fun createAllTemp() {
        for (component in tempCreate) {
            component.onCreated()
            uiComponents.add(component)
        }
        tempCreate.clear()
    }

    fun createComponent(component: Component) = tempCreate.add(component)
    fun removeComponent(component: Component) = tempRemove.add(component)
}