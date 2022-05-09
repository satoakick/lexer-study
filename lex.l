delim  [ \t\n]
ws     {delim}+
letter [A-Za-z]
digit  [0-9]
id     {letter}({letter}|{digit})*
number {digit}+(\.{digit}+)?(E[+-]?{digit}+)?

%%

{ws} {/* nop */}
if   {return(IF);}
then {return(THEN);}
else {return(ELSE)}
{id} {yylval = (int) installID(); return(ID);}
"<"  {yylval = LT; return(RELOP);}
"<=" {yylval = LE; return(RELOP);}
"="  {yylval = EQ; return(RELOP);}
"<>" {yylval = NE; return(RELOP);}
">"  {yylval = GT; return(RELOP);}
">=" {yylval = GE; return(RELOP);}

%%

fn install_id() -> usize {
  /* 
    Top of token is pointed by `yytext`.
    If length of token is inserted in yyleng,
    this function returns the pointer to entry inserting the token into the symbol table.
  */
}

fn install_num() -> usize {
  /*
    This function manage another table for const instead of `install_id()`.
  */
}
