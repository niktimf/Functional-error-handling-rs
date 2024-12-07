package pro.azhidkov.training.functional_errors

sealed interface ParseResult {
    @JvmInline
    value class Success(val value: Int) : ParseResult
    data class Failure(val pos: Int, val char: Char) : ParseResult
}

object CustomAdt {

    fun parseInt(str: String): ParseResult {
        TODO()
    }

}