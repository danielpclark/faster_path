require 'test_helper'

class PlusTest < Minitest::Test
  def defassert(result, var1, var2)
    assert_equal result, FasterPath.plus(var1, var2)
  end

  # def test_nil_inputs
  # end

  def test_it_will_plus_correctly
    defassert('/', '/', '/')
    defassert('a/b', 'a', 'b')
    defassert('a', 'a', '.')
    defassert('b', '.', 'b')
    defassert('.', '.', '.')
    defassert('/b', 'a', '/b')

    defassert('/', '/', '..')
    defassert('.', 'a', '..')
    defassert('a', 'a/b', '..')
    defassert('../..', '..', '..')
    defassert('/c', '/', '../c')
    defassert('c', 'a', '../c')
    defassert('a/c', 'a/b', '../c')
    defassert('../../c', '..', '../c')

    defassert('a//b/d//e', 'a//b/c', '../d//e')

    defassert('//foo/var/bar', '//foo/var', 'bar')
  end
end
