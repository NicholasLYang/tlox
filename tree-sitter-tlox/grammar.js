module.exports = grammar({
  name: "tlox",

  rules: {
    program: ($) => repeat($.declaration),
    declaration: ($) =>
      choice(
        $.classDeclaration,
        $.funDeclaration,
        $.varDeclaration,
        $.statement
      ),
    classDeclaration: ($) =>
      seq(
        token("class"),
        field("name", $.identifier),
        optional(seq(token("<"), field("super", $.identifier))),
        token("{"),
        repeat($.function),
        token("}")
      ),
    funDeclaration: ($) => seq(token("fun"), field("function", $.function)),
    varDeclaration: ($) =>
      seq(
        token("var"),
        $.identifier,
        token(":"),
        $.type,
        optional(seq(token("="), $.expression)),
        token(";")
      ),
    statement: ($) =>
      choice(
        $.exprStatement,
        $.forStatement,
        $.ifStatement,
        $.printStatement,
        $.returnStatement,
        $.whileStatement,
        $.block
      ),
    exprStatement: ($) => seq($.expression, token(";")),
    forStatement: ($) =>
      seq(
        token("for"),
        token("("),
        choice($.varDeclaration, $.exprStatement, token(";")),
        optional($.expression),
        token(";"),
        optional($.expression),
        token(")"),
        $.statement
      ),
    ifStatement: ($) =>
      prec.right(
        0,
        seq(
          token("if"),
          token("("),
          $.expression,
          token(")"),
          $.statement,
          optional(seq(token("else"), $.statement))
        )
      ),
    printStatement: ($) => seq(token("print"), $.expression, token(";")),
    returnStatement: ($) => seq(token("return"), $.expression, token(";")),
    whileStatement: ($) =>
      seq(token("while"), token("("), $.expression, token(")"), $.statement),
    block: ($) => seq(token("{"), repeat($.declaration), token("}")),
    expression: ($) =>
      choice(
        $.assignment,
        $.logicalExpression,
        $.equalityExpression,
        $.comparisonExpression,
        $.termExpression,
        $.factorExpression,
        $.unaryExpression,
        $.callExpression,
        $.primaryExpression
      ),
    assignment: ($) =>
      prec.right(
        0,
        seq(
          optional(seq($.callExpression, token("."))),
          $.identifier,
          token("="),
          $.assignment
        )
      ),
    logicalExpression: ($) =>
      prec.left(
        1,
        choice(
          seq($.expression, token("or"), $.expression),
          seq($.expression, token("and"), $.expression)
        )
      ),
    equalityExpression: ($) =>
      prec.left(
        2,
        choice(
          seq($.expression, token("!="), $.expression),
          seq($.expression, token("=="), $.expression)
        )
      ),
    comparisonExpression: ($) =>
      prec.left(
        3,
        choice(
          seq($.expression, token(">"), $.expression),
          seq($.expression, token(">="), $.expression),
          seq($.expression, token("<"), $.expression),
          seq($.expression, token("<="), $.expression)
        )
      ),
    termExpression: ($) =>
      prec.left(
        4,
        choice(
          seq($.expression, token("-"), $.expression),
          seq($.expression, token("+"), $.expression)
        )
      ),
    factorExpression: ($) =>
      prec.left(
        5,
        choice(
          seq($.expression, token("/"), $.expression),
          seq($.expression, token("*"), $.expression)
        )
      ),
    unaryExpression: ($) =>
      prec.right(
        6,
        choice(seq(token("-"), $.expression), seq(token("+"), $.expression))
      ),
    callExpression: ($) =>
      prec.left(
        7,
        seq(
          $.primaryExpression,
          repeat(
            choice(
              seq(token("("), optional($.arguments), token(")")),
              seq(token("."), $.identifier)
            )
          )
        )
      ),
    primaryExpression: ($) =>
      prec(
        8,
        choice(
          token("true"),
          token("false"),
          token("nil"),
          token("this"),
          $.number,
          $.string,
          $.identifier,
          seq(token("("), $.expression, token(")")),
          seq(token("super"), token("."), $.identifier)
        )
      ),
    function: ($) =>
      seq(
        field("name", $.identifier),
        token("("),
        optional(field("params", $.parameters)),
        token(")"),
        field("body", $.block)
      ),
    parameters: ($) =>
      seq(
        $.identifier,
        token(":"),
        $.type,
        repeat(seq(token(","), $.identifier, token(":"), $.type))
      ),
    arguments: ($) => seq($.expression, repeat(seq(token(","), $.expression))),
    // TODO: Expand types to include arrays
    type: ($) => $.identifier,
    number: ($) => /[0-9]+(\.[0-9]+)?/,
    string: ($) => /"[^"]*"/,
    identifier: ($) => /[A-Za-z][A-Za-z0-9]*/,
  },
});
