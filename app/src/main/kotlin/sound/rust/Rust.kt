package sound.rust

import java.io.*
import java.net.*

object Rust {
    const val PORT: Int = 13924
    const val PATH: String = "..\\RustCore"
    
    lateinit var socket: Socket
    lateinit var outputStream: OutputStream
    lateinit var printWriter: PrintWriter
    lateinit var bufferedInputStream: BufferedInputStream
    
    fun start() {
        println("Accepting")

        val serverSocket = ServerSocket(PORT)
            socket = serverSocket.accept()
            println("connected")

        outputStream = socket.getOutputStream()

        printWriter = PrintWriter(socket.getOutputStream())

        with (printWriter) {
                println("From Kotlin: 9099")
                flush()
        }
    }

    fun sendMessage(message: String?) {
        printWriter.println(message)
        printWriter.flush()
    }
}