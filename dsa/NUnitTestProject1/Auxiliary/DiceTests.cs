using System;
using DSALib.Auxiliary;
using Moq;
using NUnit.Framework;

namespace NUnitTest.Auxiliary {
    [TestFixture]
    public class DiceTests {
        [SetUp]
        public void SetUp() {
            mockRepository = new MockRepository(MockBehavior.Strict);
        }

        [TearDown]
        public void TearDown() {
            mockRepository.VerifyAll();
        }

        private MockRepository mockRepository;

        private void CreateDice() {
        }

        [Test]
        public void Roll_StateUnderTest_ExpectedBehavior() {
            // Arrange
            var d = 20;

            // Act
            var result = Dice.Roll(d);

            // Assert
            Assert.True(result > 0 && result < d + 1);
        }

        [Test]
        public void Roll_StateUnderTest_ExpectedBehavior1() {
            // Arrange
            var input = "w";

            // Act
            Assert.Throws<ArgumentException>(() => Dice.Roll(input));
        }

        [Test]
        public void Roll_zero_dice() {
            // Arrange
            var count = 0;
            var d = 2;

            // Act
            var result = Dice.Roll(
                count,
                d);

            // Assert
            Assert.AreEqual(0, result);
        }
    }
}