use core::str;
use std::collections::HashMap;

use errors::RtmlError;

pub mod errors;

#[derive(Debug)]
pub struct View<'a> {
    template: String,
    variables: Option<HashMap<&'a str, &'a str>>,
}

impl<'a> View<'a> {
    /// Creates a new `View` instance with the specified str.
    ///
    /// # Example
    ///
    /// ```
    /// use rtml::View;
    ///
    /// let template = "<html><body>{{ content }}</body></html>";
    /// let view = View::from_str(template);
    /// ```
    pub fn from_str(template: &'a str) -> Self {
        Self {
            template: template.to_string(),
            variables: None,
        }
    }

    /// Creates a new `View` instance with the specified rtml file path.
    ///
    /// # Example
    ///
    /// ```
    /// use rtml::View;
    ///
    /// let template = "views/example.rtml";
    /// let view = View::from_file(template);
    /// ```
    pub fn from_file(path: &'a str) -> Result<Self, RtmlError> {
        let template = std::fs::read_to_string(path);

        if template.is_err() {
            return Err(RtmlError::TemplateNotFound);
        }

        Ok(Self {
            template: template.unwrap(),
            variables: None,
        })
    }

    /// Adds multiple variables to the `View` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use rtml::View;
    /// use std::collections::HashMap;
    ///
    /// let template = "<html><body>{{ content }}</body></html>";
    /// let mut view = View::from_str(template);
    ///
    /// view.add_variables(vec![("content", "Hello, world!")]);
    /// ```
    pub fn add_variables(&mut self, variables: Vec<(&'a str, &'a str)>) {
        for (key, value) in variables {
            self.add_variable(key, value);
        }
    }

    /// Adds a single variable to the `View` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use rtml::View;
    ///
    /// let template = "<html><body>{{ content }}</body></html>";
    /// let mut view = View::from_str(template);
    ///
    /// view.add_variable("content", "Hello, world!");
    /// ```
    pub fn add_variable(&mut self, key: &'a str, value: &'a str) {
        match &mut self.variables {
            Some(vars) => {
                vars.insert(key, value);
            }
            None => {
                let mut vars = HashMap::new();
                vars.insert(key, value);
                self.variables = Some(vars);
            }
        }
    }

    pub fn build(self) -> Result<String, errors::RtmlError> {
        if self.template.is_empty() {
            return Err(RtmlError::EmptyTemplate);
        }

        match self.variables {
            Some(variables) => {
                let mut parsed_template = self.template.clone();
                for (key, value) in &variables {
                    let search_key = format!("{{{{ {} }}}}", key);

                    if self.template.contains(&search_key) {
                        parsed_template = parsed_template.replace(&search_key, value);
                    }
                }

                Ok(parsed_template)
            }
            None => Ok(self.template),
        }
    }
}
