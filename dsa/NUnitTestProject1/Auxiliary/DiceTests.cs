using DSALib.Auxiliary;
using Moq;
using NUnit.Framework;
using System;

namespace NUnitTest.Auxiliary
{
    [TestFixture]
    public class DiceTests
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

        private void CreateDice()
        {}

        [Test]
        public void Roll_StateUnderTest_ExpectedBehavior()
        {
            // Arrange
            int d = 20;

            // Act
            var result = Dice.Roll(d);

            // Assert
            Assert.True(result > 0 && result < d+1);
        }

        [Test]
        public void Roll_StateUnderTest_ExpectedBehavior1()
        {
            // Arrange
            string input = "w";

            // Act
            Assert.Throws<ArgumentException>( () => Dice.Roll(input));
        }

        [Test]
        public void Roll_zero_dice()
        {
            // Arrange
            int count = 0;
            int d = 2;

            // Act
            var result = Dice.Roll(
                count,
                d);

            // Assert
            Assert.AreEqual(0, result);
        }
    }
}
