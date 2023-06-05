package sound.ui

import processing.core.PVector
import sound.Processing

enum class State {
    Kill,
    Alive
}

abstract class Component(
    val sketch: Processing,
    priority: Order.Priority
): Comparable<Component> {
    private val order = Order(priority)

    open fun onCreated() {}
    abstract fun update(): State
    abstract fun display()
    open fun onDestroy() {}
    abstract fun isMouseIn(mousePosition: PVector): Boolean
    open fun mouseOut() {}
    open fun mouseIn() {}
    open fun mouseClicked() {}

    override fun compareTo(other: Component) = order.compareTo(other.order)
}

class Order(private val priority: Priority): Comparable<Order> {
    private val id = currentId++

    enum class Priority {
        Background,
        Low,
        High,
        Top
    }

    override fun compareTo(other: Order): Int {
        priority.compareTo(other.priority).let {
            if (it != 0) return it
        }

        return id.compareTo(other.id)
    }

    companion object {
        var currentId = 0
    }
}