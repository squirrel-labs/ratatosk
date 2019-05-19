namespace Firebase.Database.Offline.Internals
{
    using System.Collections.Generic;
    using System.Linq.Expressions;
    using System.Reflection;
    using Newtonsoft.Json;

    public class MemberAccessVisitor : ExpressionVisitor
    {
        private readonly IList<string> propertyNames = new List<string>();

        private bool wasDictionaryAccess;

        public IEnumerable<string> PropertyNames => propertyNames;

        public MemberAccessVisitor()
        {
        }

        public override Expression Visit(Expression expr)
        {
            if (expr?.NodeType == ExpressionType.MemberAccess)
            {
                if (wasDictionaryAccess)
                {
                    wasDictionaryAccess = false;
                }
                else
                {
                    var memberExpr = (MemberExpression) expr;
                    var jsonAttr = memberExpr.Member.GetCustomAttribute<JsonPropertyAttribute>();

                    propertyNames.Add(jsonAttr?.PropertyName ?? memberExpr.Member.Name);
                }
            }
            else if (expr?.NodeType == ExpressionType.Call)
            {
                var callExpr = (MethodCallExpression) expr;
                if (callExpr.Method.Name == "get_Item" && callExpr.Arguments.Count == 1)
                {
                    var e = Expression.Lambda(callExpr.Arguments[0]).Compile();
                    propertyNames.Add(e.DynamicInvoke().ToString());
                    wasDictionaryAccess = callExpr.Arguments[0].NodeType == ExpressionType.MemberAccess;
                }
            }

            return base.Visit(expr);
        }
    }
}