package sound.ui

import sound.Processing
import java.util.*

class UI(val sketch: Processing) {
    private val uiComponents: TreeSet<Component> = TreeSet()
    private val tempRemove = ArrayList<Component>()
    private val tempCreate = ArrayList<Component>()
    private var mouseComponent: Component? = null

    fun setup() {
        createComponent(Background(sketch))
    }

    fun draw() {
        applyAllTemp()

        for (component in uiComponents) {
            val state = component.update()

            if (state == State.Kill) {
                tempRemove.add(component)
            }

            component.display()
        }
    }

    fun mouseClicked() {
        mouseComponent = null

        for (component in uiComponents) {
            if (component.isMouseIn(sketch.mousePosition)) {
                mouseComponent = component
            }
        }

        mouseComponent?.mouseClicked()
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