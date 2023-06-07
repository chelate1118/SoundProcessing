package sound.rust

import java.io.*
import java.net.*

object Rust {
    private const val SUBPROCESS = false
    private const val PORT: Int = 13924
    private const val PATH: String = "../RustCore"
    private lateinit var printWriter: PrintWriter
    
    fun start() {
        if (SUBPROCESS) {
            Runtime.getRuntime().exec(arrayOf(
                "cargo", "run", "--manifest-path", "$PATH/Cargo.toml", "-r", "--", "$PORT"
            ))
        }

        println("Accepting")

        val serverSocket = ServerSocket(PORT)
        val socket = serverSocket.accept()
        println("connected")

        val outputStream = socket.getOutputStream()
        printWriter = PrintWriter(outputStream)
    }

    fun sendMessage(message: String?, prefix: Char) {
        printWriter.println("$prefix$message")
        printWriter.flush()
    }
}