package pro.azhidkov.training.functional_errors

import io.kotest.core.spec.style.FreeSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf
import io.kotest.property.forAll
import pro.azhidkov.training.functional_errors.CustomAdt.parseInt

class CustomAdtParserTests : FreeSpec({

    "Функция парсинга" - {

        "должна парсить 0" {
            // Дано
            val str = "0"

            // Когда
            val result = parseInt(str)

            // Тогда
            result shouldBe ParseResult.Success(0)
        }

        "должна выявлять недопустимые символы в строке" - {
            // Дано
            val nonDigit = 'o'
            val str = "1${nonDigit}1"

            // Когда
            val result = parseInt(str)

            // Тогда
            result.shouldBeInstanceOf<ParseResult.Failure>()

            "и указывать корректную позицию" {
                result.pos shouldBe str.indexOf(nonDigit)
            }

            "и корректный символ" {
                result.char shouldBe nonDigit
            }
        }

        "должна выявлять двойной минус в строке" - {
            // Дано
            val str = "--1"

            // Когда
            val result = parseInt(str)

            // Тогда
            result.shouldBeInstanceOf<ParseResult.Failure>()

            "и указывать корректную позицию" {
                result.pos shouldBe 1
            }

            "и корректный символ" {
                result.char shouldBe '-'
            }
        }

        "должна парсить положительное целое число" {
            // Дано
            val str = "1"

            // Когда
            val result = parseInt(str)

            // Тогда
            result shouldBe ParseResult.Success(1)
        }

        "должна парсить отрицательное целое число" {
            // Дано
            val str = "-1"

            // Когда
            val result = parseInt(str)

            // Тогда
            result shouldBe ParseResult.Success(-1)
        }

        "должна парсить максимально допустимый инт" {
            // Дано
            val str = Int.MAX_VALUE.toString()

            // Когда
            val result = parseInt(str)

            // Тогда
            result shouldBe ParseResult.Success(Int.MAX_VALUE)
        }

        "должна парсить минимально допустимый инт" {
            // Дано
            val str = Int.MIN_VALUE.toString()

            // Когда
            val result = parseInt(str)

            // Тогда
            result shouldBe ParseResult.Success(Int.MIN_VALUE)
        }

        "должна парсить строковое представление произвольного целого числа" {
            forAll<Int> { int ->
                parseInt(int.toString()) == ParseResult.Success(int)
            }
        }

    }

})