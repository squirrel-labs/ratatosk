using System;
using System.Globalization;

namespace Firebase.Database.Query
{
    /// <summary>
    ///     Represents a firebase filtering query, e.g. "?LimitToLast=10".
    /// </summary>
    public class FilterQuery : ParameterQuery
    {
        private readonly Func<bool> boolValueFactory;
        private readonly Func<double> doubleValueFactory;
        private readonly Func<string> valueFactory;

        /// <summary>
        ///     Initializes a new instance of the <see cref="FilterQuery" /> class.
        /// </summary>
        /// <param name="parent"> The parent. </param>
        /// <param name="filterFactory"> The filter. </param>
        /// <param name="valueFactory"> The value for filter. </param>
        /// <param name="client"> The owning client. </param>
        public FilterQuery(FirebaseQuery parent, Func<string> filterFactory, Func<string> valueFactory,
            FirebaseClient client)
            : base(parent, filterFactory, client)
        {
            this.valueFactory = valueFactory;
        }

        /// <summary>
        ///     Initializes a new instance of the <see cref="FilterQuery" /> class.
        /// </summary>
        /// <param name="parent"> The parent. </param>
        /// <param name="filterFactory"> The filter. </param>
        /// <param name="valueFactory"> The value for filter. </param>
        /// <param name="client"> The owning client. </param>
        public FilterQuery(FirebaseQuery parent, Func<string> filterFactory, Func<double> valueFactory,
            FirebaseClient client)
            : base(parent, filterFactory, client)
        {
            doubleValueFactory = valueFactory;
        }

        /// <summary>
        ///     Initializes a new instance of the <see cref="FilterQuery" /> class.
        /// </summary>
        /// <param name="parent"> The parent. </param>
        /// <param name="filterFactory"> The filter. </param>
        /// <param name="valueFactory"> The value for filter. </param>
        /// <param name="client"> The owning client. </param>
        public FilterQuery(FirebaseQuery parent, Func<string> filterFactory, Func<bool> valueFactory,
            FirebaseClient client)
            : base(parent, filterFactory, client)
        {
            boolValueFactory = valueFactory;
        }

        /// <summary>
        ///     The build url parameter.
        /// </summary>
        /// <param name="child"> The child. </param>
        /// <returns> Url parameter part of the resulting path. </returns>
        protected override string BuildUrlParameter(FirebaseQuery child)
        {
            if (valueFactory != null)
            {
                if (valueFactory() == null) return "null";
                return $"\"{valueFactory()}\"";
            }

            if (doubleValueFactory != null)
                return doubleValueFactory().ToString(CultureInfo.InvariantCulture);
            if (boolValueFactory != null) return $"{boolValueFactory().ToString().ToLower()}";

            return string.Empty;
        }
    }
}