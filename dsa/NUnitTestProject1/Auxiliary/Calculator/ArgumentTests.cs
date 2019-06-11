using DSALib.Auxiliary.Calculator;
using Moq;
using NUnit.Framework;

namespace NUnitTest.Auxiliary.Calculator
{
    [TestFixture]
    public class ArgumentTests
    {
        private MockRepository mockRepository;



        [SetUp]
        public void SetUp()
        {
            this.mockRepository = new MockRepository(MockBehavior.Strict);


        }

        [TearDown]
        public void TearDown()
        {
            this.mockRepository.VerifyAll();
        }

        private Argument CreateArgument()
        {
            return new Argument("3");
        }

        [Test]
        public void Solve_StateUnderTest_ExpectedBehavior()
        {
            // Arrange
            var unitUnderTest = this.CreateArgument();

            // Act
            var result = unitUnderTest.Solve();

            // Assert
            Assert.AreEqual(3, result);
        }

        [Test]
        public void ToString_StateUnderTest_ExpectedBehavior()
        {
            // Arrange
            var unitUnderTest = this.CreateArgument();

            // Act
            var result = unitUnderTest.ToString();

            // Assert
            Assert.AreEqual("3", result);
        }
    }
}
