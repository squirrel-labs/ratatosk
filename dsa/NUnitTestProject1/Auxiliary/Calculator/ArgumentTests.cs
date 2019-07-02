using DSALib.Auxiliary.Calculator;
using Moq;
using NUnit.Framework;

namespace NUnitTest.Auxiliary.Calculator {
    [TestFixture]
    public class ArgumentTests {
        [SetUp]
        public void SetUp() {
            mockRepository = new MockRepository(MockBehavior.Strict);
        }

        [TearDown]
        public void TearDown() {
            mockRepository.VerifyAll();
        }

        private MockRepository mockRepository;

        private Argument CreateArgument() {
            return new Argument("3");
        }

        [Test]
        public void Solve_StateUnderTest_ExpectedBehavior() {
            // Arrange
            var unitUnderTest = CreateArgument();

            // Act
            var result = unitUnderTest.Solve();

            // Assert
            Assert.AreEqual(3, result);
        }

        [Test]
        public void ToString_StateUnderTest_ExpectedBehavior() {
            // Arrange
            var unitUnderTest = CreateArgument();

            // Act
            var result = unitUnderTest.ToString();

            // Assert
            Assert.AreEqual("3", result);
        }
    }
}